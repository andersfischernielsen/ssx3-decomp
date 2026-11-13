//! `src/object/flagset.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::set_reg;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::gp as gp_3;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::intersect::worldsphtree::p;
pub(crate) use crate::game::intersect::worldsphtree::save_field_block;
pub(crate) use crate::game::intersect::worldsphtree::sp_2;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::render::lightman::gp as gp_2;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::util::menu::gp_5 as gp_4;
pub(crate) use crate::game::visualfx::boardwakefx::copy_render_state;
pub(crate) use crate::game::visualfx::boardwakefx::gp;
pub(crate) use crate::game::visualfx::boardwakefx::ldf_2 as ldf;
pub(crate) use crate::game::visualfx::boardwakefx::lh;
pub(crate) use crate::game::visualfx::boardwakefx::lw;
pub(crate) use crate::game::visualfx::boardwakefx::stf;
pub(crate) use crate::game::wscript::wscriptman::frame_at;
pub(crate) use crate::game::wscript::wscriptman::sw;
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
        0x0034_b818,
        cflag_manager_group_update,
        "cFlagManager_GroupUpdate",
    ),
    (
        0x0034_b9b0,
        flag_manager_group_render,
        "cFlagManager_GroupRender",
    ),
    (
        0x0034_c378,
        cflag_manager_group_load,
        "cFlagManager_GroupLoad",
    ),
    (0x0034_c428, c_flag_manager_ctor, "cFlagManager_ctor"),
    (0x0034_c4b8, c_flag_manager_dtor, "cFlagManager_dtor"),
    (0x0034_c548, cflag_manager_add_flag, "cFlagManager_AddFlag"),
    (0x0034_c898, c_flag_manager_load, "cFlagManager_Load"),
    (0x0034_cb80, fence_dtor, "Fence_dtor"),
    (
        0x0034_cde8,
        fence_kick_from_neighbour,
        "Fence_KickFromNeighbour",
    ),
    (0x0034_ce48, fence_update, "Fence_Update"),
    (0x0034_d650, fence_v40, "Fence_v40"),
    (0x0034_d6f0, fence_add_impulse, "Fence_AddImpulse"),
    (0x0034_d778, fence_render, "Fence_Render"),
    (0x0034_d960, fence_save, "Fence_Save"),
    (
        0x0034_dd18,
        anim_object_build_matrices,
        "AnimObject_BuildMatrices",
    ),
    (
        0x0034_e320,
        anim_object_get_node_intersect_matrix,
        "AnimObject_GetNodeIntersectMatrix",
    ),
    (0x0034_e348, anim_object_v52, "AnimObject_v52"),
    (
        0x0034_bca0,
        group_compute_verts,
        "cFlagManager_GroupComputeVerts",
    ),
    (0x0034_c668, cflag_manager_update, "cFlagManager_Update"),
    (0x0034_c7f8, flag_manager_render, "cFlagManager_Render"),
    (
        0x0034_c2e0,
        cflag_manager_group_save,
        "cFlagManager_GroupSave",
    ),
    (0x0034_dba8, anim_object_base_dtor, "AnimObject_BaseDtor"),
    (
        0x0034_c600,
        c_flag_manager_remove_flag,
        "cFlagManager_RemoveFlag",
    ),
    (0x0034_c848, c_flag_manager_save, "cFlagManager_Save"),
    (0x0034_d9b0, anim_object_base_ctor, "AnimObject_BaseCtor"),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {

    pub(crate) use crate::game::bx::bxstringctor::callee::FLAG_MANAGER_GROUP_BUILD_MESH;

    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;

    pub(crate) use crate::game::intersect::worldsphtree::callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX;

    pub(crate) use crate::game::render::lightman::callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS;

    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_AI_FRAME_COUNT;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_FLAG_MANAGER;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;

    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
}

/// The game's render-context singleton (`gp+0x2a90`, `D_004A5B80`): its
/// vtable (`+0x10d8`), the render-state stack pointer (`+0xe84`) and the
/// texture-id table (`+0xf50`).
#[allow(dead_code)]
pub(crate) mod ctx {

    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::VTABLE;
}

/// `sAmCtx` (`D_004A289C`) used by `Rider_RenderModel`, and the world
/// pointer (`gp-0x848`) whose `+0x84 -> +0x84 -> +0x14` is the active rider.
#[allow(dead_code)]
pub(crate) mod globals {
    #[cfg(test)]
    pub(crate) use crate::game::visualfx::boardwakefx::globals::IDENTITY_MAT;

    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_MANAGER;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_STATE;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_TABLE;
}

/// `cFlagManager` fields.
pub(crate) mod flag_manager {

    /// Frame parity the group was last rebuilt for (`+0x90`).
    pub const PARITY: u32 = 0x90;

    /// Vertex animation blend (`+0x4c`).
    pub const BLEND: u32 = 0x4c;

    /// Per-phase amount (`+0x4`, 4 floats) and accumulator (`+0x78`).
    pub const PHASE: u32 = 0x4;

    pub const SPEED: u32 = 0x78;

    /// Scroll speeds (`+0x50` v, `+0x54` u) and offsets (`+0x88` v, `+0x8c` u).
    pub const SCROLL_V: u32 = 0x50;

    pub const SCROLL_U: u32 = 0x54;

    pub const V: u32 = 0x88;

    pub const U: u32 = 0x8c;
}

/// A float constant read relative to `gp` (game globals / `.lit4`).
pub(crate) fn gp_float(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr((c.g[GP][0] as u32).wrapping_sub(off)).read_fpu(c)
}

// ---------------------------------------------------------------------------
// Fence.

/// `void Fence_AddImpulse(Fence *f, float impulse)` (0x34d6f0): add an impulse
/// to the fence's sway, clamping to 1.0, and damp the velocity.
pub fn fence_add_impulse(c: &mut Cpu) {
    let fence = Addr(abi::arg(c, 0));
    c.cycles += 9;
    c.f[0] = fence.off(0x30).read_fpu(c).0;
    c.f[1] = FpuF32(c.f[12]).abs().0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[2] = 0x3f80_0000;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
    let cc = FpuF32(c.f[2]).lt(FpuF32(c.f[0]));
    abi::set_fcc(c, cc);
    fence.off(0x30).write_fpu(c, FpuF32(c.f[0]));
    if cc {
        c.cycles += 1;
        fence.off(0x30).write_fpu(c, FpuF32(c.f[2]));
    }
    c.cycles += 5;
    c.f[1] = fence.off(0x34).read_fpu(c).0;
    c.f[0] = 0;
    let is_zero = FpuF32(c.f[1]).eq_s(FpuF32::ZERO);
    abi::set_fcc(c, is_zero);
    if is_zero {
        c.cycles += 4;
        let neg = FpuF32(c.f[12]).lt(FpuF32::ZERO);
        abi::set_fcc(c, neg);
        c.f[0] = fence.off(0x30).read_fpu(c).0;
        if neg {
            c.cycles += 3;
            c.f[1] = gp_float(c, 0x2ab4).0;
        } else {
            c.cycles += 2;
            c.f[1] = gp_float(c, 0x2aac).0;
        }
    } else {
        c.cycles += 4;
        let neg = FpuF32(c.f[1]).lt(FpuF32::ZERO);
        abi::set_fcc(c, neg);
        c.f[0] = fence.off(0x30).read_fpu(c).0;
        if neg {
            c.cycles += 3;
            c.f[1] = gp_float(c, 0x2ab0).0;
        } else {
            c.cycles += 2;
            c.f[1] = gp_float(c, 0x2aac).0;
        }
    }
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0;
    c.cycles += 2;
    fence.off(0x34).write_fpu(c, FpuF32(c.f[0]));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Anim object node intersection matrix.

/// `void* AnimObject_GetNodeIntersectMatrix(AnimObject *o, int node)`
/// (0x34e320): the intersect matrix of `node`, or a default when there is no
/// node table.
pub fn anim_object_get_node_intersect_matrix(c: &mut Cpu) {
    c.cycles += 3;
    let node = abi::arg(c, 1);
    let table = Addr(abi::arg(c, 0)).off(0x48).lw(c);
    c.g[V1][0] = table;
    if table == 0 {
        c.g[V0][0] = 0x500000;
        c.cycles += 2;
        c.g[V0][0] = sx(0x500000u32.wrapping_sub(0xe60));
        abi::ret(c);
    } else {
        c.cycles += 5;
        c.g[V0][0] = 0xd0;
        let off = abi::mult(c, node, 0xd0);
        c.g[A0][0] = sx(off);
        let ptr = off.wrapping_add(table as u32);
        c.g[V0][0] = sx(ptr.wrapping_add(0x90));
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Animation object: bone matrices.

/// `out = a * b` with `mulScaledFS`'s layout: the four rows of `a` times the
/// broadcast columns of `b`, transposed twice through the two scratch quads
/// (`s1`, `s2`) and stored to `dst`.
pub(crate) fn mul_scaled(c: &mut Cpu, a: u32, b: u32, dst: u32, s1: u32, s2: u32) {
    c.lqc2(4, a);
    c.lqc2(5, p(a, 0x10));
    c.lqc2(6, p(a, 0x20));
    c.lqc2(7, p(a, 0x30));
    c.lqc2(8, b);
    c.lqc2(9, p(b, 0x10));
    c.lqc2(10, p(b, 0x20));
    c.lqc2(11, p(b, 0x30));
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
    c.sqc2(12, s1);
    c.sqc2(13, p(s1, 0x10));
    c.sqc2(14, p(s1, 0x20));
    c.sqc2(15, p(s1, 0x30));
    c.lqc2(1, s1);
    c.lqc2(2, p(s1, 0x10));
    c.lqc2(3, p(s1, 0x20));
    c.lqc2(4, p(s1, 0x30));
    c.sqc2(1, s2);
    c.sqc2(2, p(s2, 0x10));
    c.sqc2(3, p(s2, 0x20));
    c.sqc2(4, p(s2, 0x30));
    c.lqc2(1, s2);
    c.lqc2(2, p(s2, 0x10));
    c.lqc2(3, p(s2, 0x20));
    c.lqc2(4, p(s2, 0x30));
    c.sqc2(1, dst);
    c.sqc2(2, p(dst, 0x10));
    c.sqc2(3, p(dst, 0x20));
    c.sqc2(4, p(dst, 0x30));
}

/// The scaled second operand of [`mul_scaled`]: the frame's scratch matrix
/// (`+0x50`, rows 0..2) from `mtx`, with row 3 replaced by the translation
/// row times the scale quad at `sp_2+0..0xc`. `fx`/`fy` are the FPU registers
/// the original keeps the x/y scale in (6 and 7, swapped per branch);
/// `r01`/`r2`/`r3` are the GPRs the original loads each row through (they are
/// left holding whole quads, so their upper halves must match too).
pub(crate) fn scaled_rows(
    c: &mut Cpu,
    mtx: u32,
    sp_2: u32,
    fx: usize,
    fy: usize,
    r01: usize,
    r2: usize,
    r3: usize,
) {
    c.lq(r01, mtx);
    c.sq(r01, p(sp_2, 0x90));
    c.sq(r01, p(sp_2, 0x50));
    c.lwc1(fx, p(sp_2, 0));
    c.lq(r01, p(mtx, 0x10));
    c.lwc1(fy, p(sp_2, 4));
    c.sq(r01, p(sp_2, 0x90));
    c.sq(r01, p(sp_2, 0x60));
    c.lwc1(5, p(sp_2, 8));
    c.lq(r2, p(mtx, 0x20));
    c.lwc1(4, p(sp_2, 0xc));
    c.sq(r2, p(sp_2, 0x90));
    c.sq(r2, p(sp_2, 0x70));
    c.lq(r3, p(mtx, 0x30));
    c.sq(r3, p(sp_2, 0xa0));
    c.lwc1(1, p(sp_2, 0xa0));
    c.lwc1(2, p(sp_2, 0xa4));
    c.lwc1(3, p(sp_2, 0xa8));
    c.mul_s(1, 1, fx);
    c.lwc1(0, p(sp_2, 0xac));
    c.mul_s(2, 2, fy);
    c.mul_s(3, 3, 5);
    c.mul_s(0, 0, 4);
    c.swc1(1, p(sp_2, 0x90));
    c.swc1(2, p(sp_2, 0x94));
    c.swc1(3, p(sp_2, 0x98));
    c.swc1(0, p(sp_2, 0x9c));
    c.lq(2, p(sp_2, 0x90));
    c.sq(2, p(sp_2, 0x80));
}

/// `void AnimObject_BuildMatrices(AnimObject* self, const sFsMtx* root,
/// sFsMtx* out)` (0x0034dd18): one matrix per skeleton bone, `out[i]`, from
/// the bone's parent (`out[parent]`, or `root` when it has none) and its
/// animation matrix (from `AnimObject_GetNodeIntersectMatrix` for animated
/// bones, the bone's own matrix otherwise), with the model scale applied to
/// the translation row.
pub fn anim_object_build_matrices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x1b0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        frame.save(c, S7, 0x130);
        frame.save(c, S0, 0x1a0);
        frame.save(c, S1, 0x190);
        frame.save(c, S2, 0x180);
        frame.save(c, S3, 0x170);
        frame.save(c, S4, 0x160);
        frame.save(c, S5, 0x150);
        frame.save(c, S6, 0x140);
        frame.save(c, FP, 0x120);
        frame.save_ra(c, 0x110);
        c.sw(p(sp_2(c), 0xf0), c.g[A0][0]);
        let model = Addr(p(c.g[A0][0] as u32, 0x2c)).lw(c);
        c.g[V0][0] = model;
        c.sw(p(sp_2(c), 0xf4), c.g[A2][0]);
        c.sw(p(sp_2(c), 0xf8), c.g[A1][0]);
        let skel = Addr(p(c.g[V0][0] as u32, 0x80)).lw(c);
        c.g[V1][0] = skel;
        c.sw(p(sp_2(c), 0x100), 0);
        c.sw(p(sp_2(c), 0xfc), c.g[V1][0]);
        c.lwc1(0, p(c.g[V0][0] as u32, 0x84)); // model scale
        c.g[S3][0] = Addr(p(c.g[V1][0] as u32, 8)).lw(c); // bones
        c.swc1(0, p(sp_2(c), 0x8));
        c.swc1(1, p(sp_2(c), 0xc));
        c.swc1(0, p(sp_2(c), 0));
        c.swc1(0, p(sp_2(c), 0x4));
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 4)).lw(c); // bone count
        let empty = c.g[V0][0] as i64 <= 0;
        c.g[S7][0] = 0;
        let spc = sp_2(c);
        if !empty {
            c.cycles += 7;
            c.g[S5][0] = c.g[A2][0];
            c.g[S2][0] = sx(p(spc, 0x10));
            c.g[S6][0] = c.g[S5][0];
            c.sw(p(spc, 0x104), c.g[S5][0]);
            c.g[S1][0] = sx(p(spc, 0xb0));
            c.g[S4][0] = sx(p(spc, 0x50));
            c.g[FP][0] = c.g[S6][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c.g[S3][0] as u32, 8)).lw(c); // animated
                let still = c.g[V0][0] == 0;
                c.g[A0][0] = u64::MAX;
                if still {
                    // Not animated: use the bone's own matrix.
                    c.cycles += 4;
                    c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0)).lw(c); // parent
                    c.g[V1][0] = u64::MAX;
                    let root_bone = c.g[A1][0] == c.g[V1][0];
                    c.g[V0][0] = Addr(p(spc, 0xf4)).lw(c); // out
                    if root_bone {
                        // out[i] = mul(root, bone->mtx * scale)
                        c.cycles += 77;
                        let mtx = Addr(p(c.g[S3][0] as u32, 0xc)).lw(c) as u32;
                        scaled_rows(c, mtx, spc, 6, 7, 2, 3, 2);
                        c.g[V1][0] = Addr(p(spc, 0xf8)).lw(c); // root
                        let dst = Addr(p(spc, 0x104)).lw(c);
                        c.g[A0][0] = dst;
                        mul_scaled(
                            c,
                            c.g[V1][0] as u32,
                            p(spc, 0x50),
                            c.g[A0][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    } else {
                        // out[i] = mul(out[parent], bone->mtx * scale)
                        c.cycles += 78;
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
                        let mtx = Addr(p(c.g[S3][0] as u32, 0xc)).lw(c) as u32;
                        c.g[A0][0] = sx(mtx);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        scaled_rows(c, mtx, spc, 7, 6, 2, 3, 2);
                        mul_scaled(
                            c,
                            c.g[A1][0] as u32,
                            p(spc, 0x50),
                            c.g[S5][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    }
                } else {
                    // Animated: the node's intersect matrix, k counts them.
                    c.cycles += 3;
                    c.g[S0][0] = Addr(p(c.g[S3][0] as u32, 0)).lw(c); // parent
                    let root_bone = c.g[S0][0] == c.g[A0][0];
                    c.g[A0][0] = Addr(p(spc, 0xf0)).lw(c); // self
                    if root_bone {
                        // out[i] = mul(root, anim_mtx * scale)
                        c.cycles += 79;
                        c.g[A1][0] = c.g[S7][0];
                        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(1));
                        callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX.call(c, 0x0034_df10)?;
                        let mtx = c.g[V0][0] as u32;
                        scaled_rows(c, mtx, spc, 6, 7, 3, 4, 3);
                        c.g[V0][0] = Addr(p(spc, 0xf8)).lw(c); // root
                        mul_scaled(
                            c,
                            c.g[V0][0] as u32,
                            p(spc, 0x50),
                            c.g[FP][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    } else {
                        // out[i] = mul(out[parent], anim_mtx * scale)
                        c.cycles += 81;
                        c.g[A1][0] = c.g[S7][0];
                        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(1));
                        callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX.call(c, 0x0034_ddcc)?;
                        let mtx = c.g[V0][0] as u32;
                        c.g[A1][0] = sx((c.g[S0][0] as u32) << 6);
                        c.g[A0][0] = Addr(p(spc, 0xf4)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        scaled_rows(c, mtx, spc, 7, 6, 3, 4, 3);
                        mul_scaled(
                            c,
                            c.g[A1][0] as u32,
                            p(spc, 0x50),
                            c.g[S6][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    }
                }
                // Next bone: advance the bone, both outputs and the counter.
                c.cycles += 13;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = Addr(p(spc, 0x100)).lw(c);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 4)).lw(c); // bone count
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = Addr(p(spc, 0x104)).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.sw(p(spc, 0x100), c.g[A0][0]);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                let more = c.g[V0][0] != 0;
                c.sw(p(spc, 0x104), c.g[V1][0]);
                if more {
                    if events::event_due(c) {
                        events::deliver_event_here(c, 0x0034_dda8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 12;
        frame.restore(c, 16, 0x1a0);
        frame.restore(c, 17, 0x190);
        frame.restore(c, 18, 0x180);
        frame.restore(c, 19, 0x170);
        frame.restore(c, 20, 0x160);
        frame.restore(c, 21, 0x150);
        frame.restore(c, 22, 0x140);
        frame.restore(c, 23, 0x130);
        frame.restore(c, FP, 0x120);
        frame.restore_ra(c, 0x110);
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
            let mut m: u32;
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

/// `void cFlagManager_GroupUpdate(cFlagManager *self)` (0x0034b818): return
/// when the manager is not set up; otherwise advance the four phase
/// animators by `blend + wind*(1-blend)`, rebuild the group's vertices when
/// the frame parity matches, and scroll `u`/`v`.
pub fn cflag_manager_group_update(c: &mut Cpu) {
    use flag_manager::*;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x280);
        c.g[V0][0] = c.ldw(gp_2(c, 0x1150));
        frame.save(c, S0, 0x270);
        frame.save_ra(c, 0x260);
        let no_manager = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        let mut tail_lq = true;
        'body: {
            if no_manager {
                break 'body;
            }
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x5c));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x270);
                tail_lq = false;
                break 'body;
            }
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x60));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x270);
                tail_lq = false;
                break 'body;
            }
            c.cycles += 14;
            callee::WORLD_GET_FLAG_MANAGER.call(c, 0x0034_b850)?;
            // t = blend + wind->rate * (1 - blend); advance the four phases.
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1);
            c.g[A0][0] = sx(off(c.g[S0][0] as u32, PHASE));
            c.lwc1(0, off(c.g[S0][0] as u32, BLEND));
            c.g[V1][0] = sx(off(c.g[S0][0] as u32, SPEED));
            c.mov_s(3, 1);
            c.lwc1(2, off(c.g[V0][0] as u32, 0x10));
            c.sub_s(1, 1, 0);
            c.g[V0][0] = 3;
            c.mul_s(2, 2, 1);
            c.add_s(2, 0, 2);
            // Flag_ReadParams already divides these amounts by the tick rate.
            loop {
                c.cycles += 8;
                c.lwc1(0, c.g[A0][0] as u32);
                c.lwc1(1, c.g[V1][0] as u32);
                c.mul_s(0, 0, 2);
                c.add_s(0, 1, 0);
                c.c_le_s(3, 0);
                let no_wrap = !c.fcc();
                c.swc1(0, c.g[V1][0] as u32);
                if !no_wrap {
                    c.cycles += 2;
                    c.sub_s(0, 0, 3);
                    c.swc1(0, c.g[V1][0] as u32);
                }
                c.cycles += 4;
                c.g[A0][0] = sx(off(c.g[A0][0] as u32, 4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                let more = (c.g[V0][0] as i64) >= 0;
                c.g[V1][0] = sx(off(c.g[V1][0] as u32, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_b880);
                }
            }
            // Frame parity: rebuild the group's vertices when it matches.
            c.cycles += 10;
            callee::WORLD_GET_AI_FRAME_COUNT.call(c, 0x0034_b8c0)?;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
            c.g[A0][0] = c.ldw(off(c.g[S0][0] as u32, PARITY));
            c.g[V1][0] = addu(c.g[V0][0], c.g[V1][0]);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            if c.g[V0][0] != c.g[A0][0] {
                c.lwc1(1, off(c.g[S0][0] as u32, SCROLL_V));
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x31;
                c.g[V1][0] = u64::MAX;
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    let again = c.g[V0][0] != c.g[V1][0];
                    c.g[A0][0] = c.g[S0][0];
                    if !again {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_b8e8);
                    }
                }
                c.cycles += 10;
                c.g[A1][0] = c.g[SP][0];
                callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS.call(c, 0x0034_b90c)?;
                c.g[A2][0] = c.ldw(off(c.g[S0][0] as u32, 0x60));
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.ldw(off(c.g[A2][0] as u32, 4));
                c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x18));
                c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x1c));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = addu(c.g[A2][0], c.g[A0][0]);
                call_ptr(c, target, 0x0034_b928)?;
                c.lwc1(1, off(c.g[S0][0] as u32, SCROLL_V));
            }
            // Scroll the texture offsets.
            c.cycles += 5;
            c.lwc1(2, gp_2(c, -0x2b00));
            c.c_lt_s(2, 1);
            if c.fcc() {
                c.lwc1(0, off(c.g[S0][0] as u32, V));
            } else {
                c.cycles += 5;
                c.lwc1(0, off(c.g[S0][0] as u32, SCROLL_U));
                c.c_lt_s(2, 0);
                if !c.fcc() {
                    frame.restore(c, S0, 0x270);
                    tail_lq = false;
                    break 'body;
                }
                c.cycles += 1;
                c.lwc1(0, off(c.g[S0][0] as u32, V));
            }
            c.cycles += 7;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            // Flag_ReadParams also converts the scroll speeds to per-tick steps.
            c.add_s(0, 0, 1);
            c.c_lt_s(2, 0);
            let no_wrap = !c.fcc();
            c.swc1(0, off(c.g[S0][0] as u32, V));
            if !no_wrap {
                c.cycles += 2;
                c.sub_s(0, 0, 2);
                c.swc1(0, off(c.g[S0][0] as u32, V));
            }
            c.cycles += 7;
            c.lwc1(1, off(c.g[S0][0] as u32, U));
            c.lwc1(0, off(c.g[S0][0] as u32, SCROLL_U));
            c.add_s(0, 1, 0);
            c.c_lt_s(2, 0);
            let no_wrap = !c.fcc();
            c.swc1(0, off(c.g[S0][0] as u32, U));
            if !no_wrap {
                c.cycles += 2;
                c.sub_s(0, 0, 2);
                c.swc1(0, off(c.g[S0][0] as u32, U));
            }
        }
        if tail_lq {
            c.cycles += 1;
            frame.restore(c, S0, 0x270);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x260);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimObject_v52(AnimObject *self)` (0x0034e348): move every flag
/// element's vertices to `self->time` (virtual slot 2 per element), then
/// mark the object dirty.
pub fn anim_object_v52(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x48));
        let has_items = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if has_items {
            c.cycles += 3;
            c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x40));
            let no_items = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0;
            if !no_items {
                c.cycles += 2;
                c.g[S2][0] = 0;
                c.g[A1][0] = c.ldw(off(c.g[S0][0] as u32, 0x48));
                loop {
                    c.cycles += 13;
                    c.g[S1][0] = sx(off(c.g[S1][0] as u32, 1));
                    c.lwc1(12, off(c.g[S0][0] as u32, 4));
                    c.g[A1][0] = addu(c.g[S2][0], c.g[A1][0]);
                    c.g[V0][0] = c.ldw(off(c.g[A1][0] as u32, 0x84));
                    c.g[S2][0] = sx(off(c.g[S2][0] as u32, 0xd0));
                    c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x10));
                    c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x14));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = addu(c.g[A1][0], c.g[A0][0]);
                    call_ptr(c, target, 0x0034_e3a4)?;
                    c.g[V0][0] = c.ldw(off(c.g[S0][0] as u32, 0x40));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = c.ldw(off(c.g[S0][0] as u32, 0x48));
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_e380);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x20);
        c.stw(off(c.g[S0][0] as u32, 0x10), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Fence_Save(Fence* self, Stream* stream)` (0x34d960): 0x24 bytes at
/// `+0x1c`.
pub fn fence_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(
            c,
            callee::WORLD_OBJ_SAVE,
            0x1c,
            0x24,
            0x0034_d97c,
            0x0034_d998,
        )
    })
}

/// `void cFlagManager_Load(cFlagManager* self, void* arg)` (0x34c898): load
/// the 15 flag groups that follow the header.
pub fn c_flag_manager_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let mut group = r32(c, A0).wrapping_add(0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(group);
        c.g[S1][0] = 0xe;
        c.g[S2][0] = c.g[A1][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = sx(group);
            c.g[RA][0] = sx(0x0034_c8c4);
            c.g[A1][0] = c.g[S2][0];
            callee::C_FLAG_MANAGER_GROUP_LOAD.call(c, 0x0034_c8c4)?;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            group = r32(c, S0).wrapping_add(0x188);
            c.g[S0][0] = sx(group);
            if !more {
                c.cycles += 6;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_c8b8);
            }
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Fences and riders.

/// The delay before a fence passes its wobble on to a neighbour (`+0x38`
/// towards `+0x104`, `+0x3c` towards `+0x108`), counted down once per
/// update by [`fence_update`]: 4 frames at 60 Hz. Stored in ticks so the
/// wobble travels along the fence at the same speed at any rate (4 at 60).
fn fence_kick_delay(c: &mut Cpu) -> u64 {
    crate::tick::ticks_from_frames60(c, 4) as u64
}

/// `void Fence_Update(Fence* self)` (0x34ce48, vtable slot 2): once per
/// tick for a fence that was hit.
///
/// - The neighbour delays `+0x38` / `+0x3c` count down; when one runs out
///   the wobble is passed on (`Fence_PropagateToNeighbour(self, +0x104, 1)`
///   / `(self, +0x108, 0)`).
/// - The sway angle `+0x2c` moves by the velocity `+0x34`. When the
///   amplitude `+0x30` and `|angle|` are both below `gp-0x2ac8` (0.01) the
///   fence goes to sleep (its interface's slot at `+0x110`, arg 1) and the
///   vertices are left alone. Otherwise the angle bounces at `±amplitude`:
///   it is clamped there, the amplitude halves, and the velocity turns
///   round as `∓0.1 * amplitude` (`gp-0x2ac4` / `gp-0x2ac0`); then the
///   vertices are rebuilt (`Fence_UpdateVerts`).
///
/// Tick rate: the velocity is per 60 Hz frame (no step), so above 60 the
/// angle moves by `velocity * 60/rate` per tick; the bounce and halving
/// happen per swing and need nothing. The delays are set in ticks by
/// [`fence_kick_delay`].
pub fn fence_update(c: &mut Cpu) {
    const PROPAGATE: Callee = Callee {
        addr: 0x0034_cc80,
        f: ee::s11::f_0034cc80,
    };
    const UPDATE_VERTS: Callee = Callee {
        addr: 0x0034_cf98,
        f: ee::s11::f_0034cf98,
    };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let fence = r32(c, S0);

        // Neighbour delays.
        let left = Addr(fence).off(0x38).lw(c);
        c.g[V0][0] = sx((left as u32).wrapping_sub(1));
        if (left as i64) > 0 {
            c.cycles += 2;
            Addr(fence).off(0x38).sw(c, c.g[V0][0]);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[A1][0] = Addr(fence).off(0x104).lw(c);
                c.g[A2][0] = 1;
                PROPAGATE.call(c, 0x0034_ce78)?;
            }
        }
        c.cycles += 3;
        let right = Addr(fence).off(0x3c).lw(c);
        c.g[V0][0] = sx((right as u32).wrapping_sub(1));
        if (right as i64) > 0 {
            c.cycles += 2;
            Addr(fence).off(0x3c).sw(c, c.g[V0][0]);
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = Addr(fence).off(0x108).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0;
                PROPAGATE.call(c, 0x0034_ce9c)?;
            }
        }

        // Sway: angle += velocity.
        c.cycles += 10;
        c.lwc1(0, fence + 0x2c);
        c.g[V0][0] = sx(fence + 0x1c);
        c.lwc1(1, fence + 0x34);
        c.lwc1(2, fence + 0x30);
        c.lwc1(3, gp(c, -0x2ac8));
        if crate::tick::rate(c) == 60 {
            c.add_s(0, 0, 1);
        } else {
            let step = crate::game::ai::rider::rate_step_60(c, FpuF32(c.f[1]));
            c.f[0] = (FpuF32(c.f[0]) + step).0;
        }
        c.c_lt_s(2, 3);
        let small = c.fcc();
        c.swc1(0, fence + 0x2c);
        if small {
            c.cycles += 6;
            c.lwc1(0, r32(c, V0) + 0x10);
            c.abs_s(0, 0);
            c.c_lt_s(0, 3);
            c.g[A1][0] = 1;
            if c.fcc() {
                // Settled: put the fence to sleep.
                c.cycles += 7;
                c.g[V0][0] = Addr(fence).off(0xc).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).off(0x110).lh(c);
                c.g[V1][0] = Addr(r32(c, V0)).off(0x114).lw(c);
                c.g[A0][0] = sx(fence.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, r32(c, V1), 0x0034_cef0)?;
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }

        // Bounce at ±amplitude.
        c.cycles += 6;
        c.lwc1(1, fence + 0x34);
        c.mtc1(0, 0);
        c.c_le_s(0, 1);
        let mut bounced = false;
        if c.fcc() {
            // Moving up: clamp at +amplitude.
            c.cycles += 5;
            c.lwc1(0, fence + 0x2c);
            c.lwc1(1, fence + 0x30);
            c.c_le_s(1, 0);
            if c.fcc() {
                c.cycles += 9;
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(AT, 0);
                c.swc1(1, fence + 0x2c);
                c.mul_s(0, 1, 0);
                c.lwc1(2, gp(c, -0x2ac4));
                c.neg_s(1, 0);
                c.swc1(0, fence + 0x30);
                c.mul_s(1, 1, 2);
                bounced = true;
            }
        } else {
            // Moving down: clamp at -amplitude.
            c.lwc1(3, fence + 0x30);
            c.cycles += 5;
            c.lwc1(0, fence + 0x2c);
            c.neg_s(2, 3);
            c.c_le_s(0, 2);
            if c.fcc() {
                c.cycles += 7;
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(AT, 0);
                c.lwc1(1, gp(c, -0x2ac0));
                c.mul_s(0, 3, 0);
                c.swc1(2, fence + 0x2c);
                c.mul_s(1, 0, 1);
                c.swc1(0, fence + 0x30);
                bounced = true;
            }
        }
        if bounced {
            c.cycles += 1;
            c.swc1(1, fence + 0x34);
        }

        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        UPDATE_VERTS.call(c, 0x0034_cf84)?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Fence_KickFromNeighbour(Fence* self, int on)` (0x34cde8): give the
/// fence an impulse and arm the kick timer for whichever side was hit.
///
/// Tick rate: the timer is [`fence_kick_delay`] (4 frames at 60 Hz).
pub fn fence_kick_from_neighbour(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0034_d6f0, 0x0034_ce04)?;
        if c.g[S0][0] == 0 {
            c.g[V0][0] = lw_2(c, p_2(c.g[S1][0], 0x3c));
            c.cycles += 2;
            let armed = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x20);
            if !armed {
                c.cycles += 2;
                c.g[V0][0] = fence_kick_delay(c);
                Addr(p_2(c.g[S1][0], 0x3c)).sw(c, c.g[V0][0]);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw_2(c, p_2(c.g[S1][0], 0x38));
            let armed = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x20);
            if !armed {
                c.cycles += 3;
                c.g[V0][0] = fence_kick_delay(c);
                Addr(p_2(c.g[S1][0], 0x38)).sw(c, c.g[V0][0]);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Fence_dtor(Fence* self, int flags)` (0x34cb80): install the deleting
/// vtable, release the handler object through the world's interface (slot
/// 77), then run the `WorldObj` destructor.
pub fn fence_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.g[V0][0] = 0x490000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x600));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        let none = c.g[V1][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 7;
            c.g[A2][0] = c.lw(gp_3(c, 0x2a90));
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x268));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x26c));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0034_cbc8);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0034_cbc8)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_fbf0, 0x0034_cbd4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Flag manager.

/// `cFlagManager* cFlagManager_ctor(cFlagManager* self)` (0x34c428): process
/// base, then fourteen flag groups and the blend constants.
pub fn c_flag_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(p(r32(c, S0), 0x20));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xe;
        callee::GAME_PROCESS_CTOR.call(c, 0x0034_c450)?;
        c.g[S3][0] = u64::MAX;
        c.g[V0][0] = 0x0049_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x480));
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
            callee::C_FLAG_MANAGER_GROUP_CTOR.call(c, 0x0034_c46c)?;
            let more = c.g[S2][0] != c.g[S3][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x188));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_c460);
            }
        }
        c.cycles += 16;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = c.g[S0][0];
        c.g[AT][0] = 0x3e80_0000;
        c.mtc1(1, 0);
        c.swc1(1, p(r32(c, S0), 0x14));
        c.swc1(0, p(r32(c, S0), 0x18));
        Addr(p(r32(c, S0), 0x1c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x10)).sw(c, 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFlagManager_dtor(cFlagManager* self, int flags)` (0x34c4b8): destroy
/// the embedded flag set's elements backwards, then the game process.
pub fn c_flag_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x480));
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x40);
        c.g[V1][0] = sx(p(self_, 0x20));
        frame.save(c, S2, 0x20);
        c.g[S3][0] = flags;
        frame.save_ra(c, 0);
        let no_elems = c.g[V1][0] == 0;
        c.sw(at(c, S1, 0xc), c.g[V0][0]);
        if !no_elems {
            c.cycles += 3;
            c.g[S0][0] = sx(p(r32(c, S1), 0x1718));
            let empty = c.g[V1][0] == c.g[S0][0];
            c.g[S2][0] = c.g[V1][0];
            if !empty {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x188));
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = c.lw(at(c, S0, 0x184));
                    c.g[A0][0] = c.lh(at(c, V0, 8));
                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0034_c518)?;
                    let more = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x188));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_c500);
                    }
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, callee::GAME_PROCESS_DTOR, 0x0034_c52c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Fences.

/// `void Fence_v40(Fence* self, Vec4* impulse)` (0x34d650): when the fence is
/// below full strength, add a scaled impulse to it and count the hit (up to
/// four in each of two counters).
pub fn fence_v40(c: &mut Cpu) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.sq(16, frame_at(c, 0x10));
    c.sd(frame_at(c, 0), c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    c.lwc1(0, p(c.g[S0][0] as u32, 0x30));
    c.c_le_s(1, 0);
    if c.fcc() {
        c.lq(16, frame_at(c, 0x10));
    } else {
        c.cycles += 17;
        c.lwc1(1, gp(c, -0x2ab8));
        c.lqc2(3, p(c.g[A1][0] as u32, 0x20));
        c.lqc2(5, p(c.g[A1][0] as u32, 0x10));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 12);
        c.lwc1(0, p(c.g[A1][0] as u32, 0x30));
        c.mul_s(12, 12, 1);
        c.mul_s(12, 12, 0);
        if call_ptr(c, 0x0034_d6f0, 0x0034_d6b4).is_err() {
            return;
        } // Fence_AddImpulse
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x38));
        if c.g[V0][0] != 0 {
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x3c));
        } else {
            c.cycles += 3;
            c.g[V0][0] = fence_kick_delay(c);
            sw(c, p(c.g[S0][0] as u32, 0x38), c.g[V0][0]);
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x3c));
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.lq(16, frame_at(c, 0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = fence_kick_delay(c);
            sw(c, p(c.g[S0][0] as u32, 0x3c), c.g[V0][0]);
            c.lq(16, frame_at(c, 0x10));
        }
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld(frame_at(c, 0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// `cFlagManager_GroupLoad` (0x34c378): let the stream create the group's mesh
// data, fill each group item from the stream, then build the mesh for the
// first item.
pub fn cflag_manager_group_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x60;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0u64;
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw(c.g[S3][0] as u32);
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_c3b8)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x94));
        if !empty {
            c.cycles += 1;
            loop {
                c.cycles += 11;
                c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0034_c3e0)?;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_c3c8);
                }
            }
        }
        c.cycles += 2;
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S1][0];
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x94));
            callee::FLAG_MANAGER_GROUP_BUILD_MESH.call(c, 0x0034_c408)?;
        }
        c.cycles += 7;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFlagManager_AddFlag(self, group, flag)` (0x34c548): add a flag to
/// the matching group, or to the first group with room.
pub fn cflag_manager_add_flag(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0034_c548 => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x60);
                    frame.save(c, S4, 0x10);
                    frame.save(c, S0, 0x50);
                    frame.save(c, S1, 0x40);
                    frame.save(c, S2, 0x30);
                    frame.save(c, S3, 0x20);
                    frame.save_ra(c, 0);
                    let this = c.g[A0][0];
                    c.g[S4][0] = this;
                    c.g[S0][0] = sx((this as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A2][0];
                    c.g[S2][0] = 0;
                    c.g[S3][0] = c.g[A1][0];
                    pc = 0x0034_c578;
                }
                0x0034_c578 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5c)).lw(c);
                    let empty = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    pc = if empty { 0x0034_c598 } else { 0x0034_c584 };
                }
                0x0034_c584 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0034_afe8, 0x0034_c590)?;
                    let matched = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    pc = if matched { 0x0034_c5bc } else { 0x0034_c598 };
                }
                0x0034_c598 => {
                    c.cycles += 4;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 15) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x188));
                    if !more {
                        pc = 0x0034_c5a8;
                    } else {
                        if crate::events::back_edge_unwinds(c, 0x0034_c578) {
                            return Ok(());
                        }
                        pc = 0x0034_c578;
                    }
                }
                0x0034_c5a8 => {
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = 0;
                    pc = 0x0034_c5b0;
                }
                0x0034_c5b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x5c)).lw(c);
                    let busy = c.g[V0][0] != 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    pc = if busy { 0x0034_c5d0 } else { 0x0034_c5bc };
                }
                0x0034_c5bc => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0034_b038, 0x0034_c5c8)?;
                    c.lq(16, p(c.g[SP][0] as u32, 0x50));
                    pc = 0x0034_c5e0;
                }
                0x0034_c5d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 15) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x188));
                    if !more {
                        pc = 0x0034_c5dc;
                    } else {
                        if crate::events::back_edge_unwinds(c, 0x0034_c5b0) {
                            return Ok(());
                        }
                        pc = 0x0034_c5b0;
                    }
                }
                0x0034_c5dc => {
                    c.cycles += 1;
                    c.lq(16, p(c.g[SP][0] as u32, 0x50));
                    pc = 0x0034_c5e0;
                }
                0x0034_c5e0 => {
                    c.cycles += 7;
                    c.lq(17, p(c.g[SP][0] as u32, 0x40));
                    c.lq(18, p(c.g[SP][0] as u32, 0x30));
                    c.lq(19, p(c.g[SP][0] as u32, 0x20));
                    c.lq(20, p(c.g[SP][0] as u32, 0x10));
                    c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
                    let ret = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.pc = ret;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0034_c548, 0x0034_c5fc);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// 0x0034d778 Fence_Render
// ---------------------------------------------------------------------------

/// `int Fence_Render(Fence* self)` (0x34d778).
///
/// Returns 1 when the fence's board object is not in the "hidden" state; 0
/// when the rider's board model is not the active type or the visual state
/// mismatches; otherwise builds the render object for the fence's locator and
/// returns the result of the draw call. `v1` carries the last visual-state word
/// on the early-out paths.
pub fn fence_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xd0);
        c.g[V0][0] = c.lw(gp_4(c, 0x16c8));
        frame.save(c, 21, 0x70);
        frame.save(c, 16, 0xc0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, 17, 0xb0);
        c.g[A0][0] = 0xff;
        frame.save(c, 18, 0xa0);
        frame.save(c, 19, 0x90);
        frame.save(c, 20, 0x80);
        frame.save_ra(c, 0x60);

        'ret: {
            c.g[A1][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = c.lbu(p(c.g[A1][0] as u32, 0x7d));
            let t = c.g[V1][0] == c.g[A0][0];
            c.g[V0][0] = 1;
            if t {
                break 'ret;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lbu(p(c.g[A1][0] as u32, 0x7d));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx(c.g[A2][0] as u32 + c.g[V0][0] as u32);
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x24));
            c.g[V1][0] = c.g[V1][0] ^ 6;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 0;
                break 'ret;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lh(p(c.g[A1][0] as u32, 0x7e));
            c.g[A0][0] = 0x18;
            c.mult(0, 3, 2, 4);
            c.g[V0][0] = sx(c.g[V1][0] as u32 + c.g[A2][0] as u32);
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x3f0));
            c.g[V1][0] = c.g[V1][0] ^ 3;
            c.g[V0][0] = (c.g[V1][0] < 1) as u64;
        }

        'body: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0;
            c.g[S4][0] = c.lw(gp_4(c, 0x2a90));
            if t {
                break 'body;
            }
            c.cycles += 37;
            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
            c.g[A0][0] = c.g[S2][0];
            c.lwc1(0, p(c.g[V0][0] as u32, 0x60));
            c.g[S0][0] = c.lh(p(c.g[S1][0] as u32, 0x2e8));
            c.swc1(0, c.g[SP][0] as u32);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.g[S0][0] = sx(c.g[S4][0] as u32 + c.g[S0][0] as u32);
            c.lwc1(0, p(c.g[V0][0] as u32, 0x64));
            c.swc1(0, p(c.g[SP][0] as u32, 4));
            c.lwc1(1, p(c.g[V0][0] as u32, 0x68));
            c.swc1(2, p(c.g[SP][0] as u32, 0xc));
            c.swc1(1, p(c.g[SP][0] as u32, 8));
            c.lwc1(0, p(c.g[V0][0] as u32, 0x6c));
            c.swc1(0, p(c.g[SP][0] as u32, 0x10));
            c.lwc1(1, p(c.g[V0][0] as u32, 0x70));
            c.swc1(1, p(c.g[SP][0] as u32, 0x14));
            c.lwc1(0, p(c.g[V0][0] as u32, 0x74));
            c.swc1(2, p(c.g[SP][0] as u32, 0x1c));
            c.swc1(0, p(c.g[SP][0] as u32, 0x18));
            c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x158));
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x15c));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx(c.g[S4][0] as u32 + c.g[A1][0] as u32);
            call_ptr(c, t, 0x0034_d868)?;
            c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x2ec));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S2][0];
            let t = c.g[V0][0] as u32;
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, t, 0x0034_d880)?;
            c.g[V1][0] = 1;
            let t = c.g[V0][0] == c.g[V1][0];
            c.g[A1][0] = 0xffff_ffff_ffff_ff83;
            if t {
                break 'body;
            }
            'inner: {
                'no_anim: {
                    c.cycles += 22;
                    c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                    c.ldl(4, p(c.g[V0][0] as u32, 7));
                    c.ldr(4, c.g[V0][0] as u32);
                    c.ldl(6, p(c.g[V0][0] as u32, 0xf));
                    c.ldr(6, p(c.g[V0][0] as u32, 8));
                    c.g[A3][0] = c.lw(p(c.g[V0][0] as u32, 0x10));
                    c.sdl(4, p(c.g[V0][0] as u32, 0x1b));
                    c.sdr(4, p(c.g[V0][0] as u32, 0x14));
                    c.sdl(6, p(c.g[V0][0] as u32, 0x23));
                    c.sdr(6, p(c.g[V0][0] as u32, 0x1c));
                    c.sw(p(c.g[V0][0] as u32, 0x24), c.g[A3][0]);
                    c.g[A0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                    c.g[V1][0] = sx(c.g[A0][0] as u32 + 0x14);
                    c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V1][0]);
                    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x18));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | 0x14;
                    c.sw(p(c.g[A0][0] as u32, 0x18), c.g[V0][0]);
                    c.g[A0][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
                    c.g[T0][0] = c.lw(p(c.g[S5][0] as u32, 0x40));
                    let t = c.g[T0][0] == 0;
                    c.g[A1][0] = sx(c.g[A0][0] as u32 + 0x10);
                    if t {
                        break 'no_anim;
                    }
                    c.cycles += 9;
                    c.g[V0][0] = c.lw(p(c.g[T0][0] as u32, 4));
                    c.g[A3][0] = 0;
                    c.g[A2][0] = c.lw(p(c.g[A0][0] as u32, 0x94));
                    c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x38));
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(c.g[T0][0] as u32 + c.g[A0][0] as u32);
                    call_ptr(c, t, 0x0034_d900)?;
                    let t = true;
                    c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                    if t {
                        break 'inner;
                    }
                }
                c.cycles += 10;
                c.g[V1][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
                c.g[A1][0] = c.g[A0][0];
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x300));
                c.g[T0][0] = 0;
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x304));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(c.g[S4][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_d92c)?;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                break 'inner;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
            c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V0][0]);
        }

        c.cycles += 9;
        frame.restore(c, 16, 0xc0);
        frame.restore(c, 17, 0xb0);
        frame.restore(c, 18, 0xa0);
        frame.restore(c, 19, 0x90);
        frame.restore(c, 20, 0x80);
        frame.restore(c, 21, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFlagManager_GroupComputeVerts (0x0034bca0)

/// `FlagManager* World_GetFlagManager()` (0x002d1cb0, called through the
/// function table as the original does).
pub(crate) const WORLD_GET_FLAG_MANAGER: u32 = 0x002d_1cb0;

/// `this + off` (or any register plus an offset), as the compiled address
/// arithmetic does it (32-bit wrap).
#[inline(always)]
pub(crate) fn ad(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// The sine/cosine slots of the 512-entry wave table at `base` for the
/// angle index in v0: `v0 = &table[i & 0x1ff]`, `v1 = &table[(i & 0x1ff) +
/// 0x80]` (a quarter turn later; the table has 0x80 extra entries).
#[inline(always)]
pub(crate) fn wave_slots(c: &mut Cpu, base: usize) {
    c.g[V0][0] &= 0x1ff;
    c.g[V1][0] = sx(ad(c, V0, 0x80));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[base][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[base][0] as u32));
}

/// Flag group fields (`cFlagGroup`, s0): style bytes, wave phases and
/// amplitudes, grid size, rest positions.
pub(crate) const GROUP_PINNED: u32 = 0x00;

// byte; with +1 also a u16 "animated" test
pub(crate) const GROUP_MIRROR: u32 = 0x01;

// byte: 1 = flip the columns
pub(crate) const GROUP_EDGE_ONLY: u32 = 0x02;

// byte: freeze the first row
pub(crate) const GROUP_COLUMNS: u32 = 0x64;

pub(crate) const GROUP_ROWS: u32 = 0x68;

pub(crate) const GROUP_COLUMN_SPAN: u32 = 0x6c;

pub(crate) const GROUP_ROW_SPAN: u32 = 0x70;

pub(crate) const GROUP_REST_VERTS: u32 = 0x74;

pub(crate) const GROUP_WIND_BLEND: u32 = 0x4c;

/// `void cFlagManager_GroupComputeVerts(cFlagGroup* group, Vec3* verts)`
/// (0x0034bca0): animate a cloth flag. Four wave phases (+0x78, scaled by
/// a `.lit4` constant) and four amplitudes (+0x14, scaled by the wind
/// blend of the flag manager) drive a per-column offset table on the
/// stack; then every grid vertex is the rest vertex plus its column's
/// offset plus two travelling ripples weighted by the row. Pinned groups
/// fold the weight at the middle (`> 0.5` mirrors); the mirror style flips
/// the column order; the edge-only style copies the first row unchanged.
/// The register-level FPU ops and scratch registers follow the original
/// exactly (the wave table at 0x504fb8, `cvt.w.s` indices).
pub fn group_compute_verts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x120);
        'flags: {
            frame.save(c, S0, 0x110);
            frame.save(c, S2, 0xf0);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S3, 0xe0);
            c.g[S2][0] = 0;
            frame.save(c, S1, 0x100);
            frame.save_ra(c, 0xd0);
            c.g[V0][0] = c.lb(ad(c, S0, GROUP_PINNED));
            c.g[S3][0] = c.g[A1][0];
            if c.g[V0][0] == 0 {
                break 'flags;
            }
            c.cycles += 2;
            c.g[V0][0] = c.lb(ad(c, S0, GROUP_MIRROR));
            // s2: pinned and mirrored (the "fold at the middle" style).
            c.g[S2][0] = (c.g[V0][0] != 0) as u64;
        }
        c.cycles += 17;
        c.g[S1][0] = c.lhu(ad(c, S0, GROUP_PINNED)) as u64;
        call_ptr(c, WORLD_GET_FLAG_MANAGER, 0x0034_bcdc)?;

        // Wind blend: f12 = blend + manager.wind * (1 - blend).
        c.lwc1(2, ad(c, S0, GROUP_WIND_BLEND));
        c.g[S1][0] = (c.g[S1][0] < 1) as u64; // s1: not animated
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.g[A1][0] = sx(ad(c, SP, 0x10));
        c.lwc1(0, ad(c, V0, 0x10));
        c.g[A0][0] = sx(ad(c, S0, 0x14));
        c.sub_s(1, 1, 2);
        c.lwc1(3, ad(c, GP, 0u32.wrapping_sub(0x2afc)));
        c.g[V1][0] = c.g[SP][0];
        c.g[V0][0] = sx(ad(c, S0, 0x78));
        c.g[A2][0] = 3;
        c.mul_s(0, 0, 1);
        c.add_s(12, 2, 0);
        // Four phases (sp+0x00) and amplitudes (sp+0x10).
        loop {
            c.cycles += 12;
            c.lwc1(1, ad(c, V0, 0));
            c.g[A2][0] = sx(ad(c, A2, 0xffff_ffff));
            c.lwc1(0, ad(c, A0, 0));
            c.g[V0][0] = sx(ad(c, V0, 4));
            c.mul_s(1, 1, 3);
            c.g[A0][0] = sx(ad(c, A0, 4));
            c.mul_s(0, 0, 12);
            c.swc1(1, ad(c, V1, 0));
            c.swc1(0, ad(c, A1, 0));
            c.g[V1][0] = sx(ad(c, V1, 4));
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A1][0] = sx(ad(c, A1, 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_bd18);
            }
        }

        // Per-column offsets (sp+0x20, 12 bytes per column).
        c.cycles += 9;
        c.g[A0][0] = c.lw(ad(c, S0, GROUP_COLUMNS));
        c.g[V0][0] = 9;
        c.g[V1][0] = u64::MAX;
        c.mtc1(0, 13);
        c.g[A2][0] = 0;
        c.mov_s(16, 13);
        c.mov_s(15, 13);
        if (c.g[A0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 14);
            loop {
                column_offsets(c);
                c.g[V0][0] = c.lw(ad(c, S0, GROUP_COLUMNS));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.add_s(13, 13, 14);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_bd78);
                }
            }
        }

        // The vertex grid.
        c.cycles += 5;
        c.g[V0][0] = c.lw(ad(c, S0, GROUP_ROWS));
        c.g[T2][0] = 0;
        c.mtc1(0, 18);
        c.g[V1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[T4][0] = 0xc;
            c.g[T1][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw(ad(c, S0, GROUP_COLUMNS));
                c.g[T3][0] = sx(ad(c, V1, 1));
                c.mtc1(0, 13);
                c.g[A3][0] = 0;
                if (c.g[V0][0] as i64) > 0 {
                    c.cycles += 13;
                    c.mult(0, A2, T2, T4);
                    c.g[V0][0] = 0x50_0000;
                    c.g[AT][0] = 0x3f00_0000;
                    c.mtc1(AT, 11);
                    c.g[T0][0] = sx(ad(c, V0, 0x4fb8));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 12);
                    c.mov_s(14, 13);
                    c.lwc1(17, ad(c, GP, 0u32.wrapping_sub(0x2af0)));
                    c.g[A1][0] = sx(ad(c, SP, 0x20));
                    c.lwc1(16, ad(c, GP, 0u32.wrapping_sub(0x2aec)));
                    c.g[A0][0] = sx(ad(c, T1, c.g[S3][0] as u32));
                    c.lwc1(15, ad(c, GP, 0u32.wrapping_sub(0x2ae8)));
                    loop {
                        grid_vertex(c);
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(ad(c, S0, GROUP_COLUMNS));
                        c.g[A3][0] = sx(ad(c, A3, 1));
                        c.g[A1][0] = sx(ad(c, A1, 0xc));
                        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.add_s(13, 13, 12);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0034_c030);
                        }
                    }
                }
                c.cycles += 7;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 0);
                c.g[V1][0] = c.g[T3][0];
                c.g[V0][0] = c.lw(ad(c, S0, GROUP_ROWS));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.add_s(18, 18, 0);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_bfe8);
                }
            }
        }

        c.cycles += 7;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// One column of the offset table (`a2` = column, f13 = column number as
/// a float, f14 = 1.0): its weight `f3`, the two travelling waves and the
/// static shape terms, into the 12-byte slot of the (possibly mirrored)
/// column index `a0`.
pub(crate) fn column_offsets(c: &mut Cpu) {
    'weighted: {
        c.cycles += 5;
        c.lwc1(0, ad(c, S0, GROUP_COLUMN_SPAN));
        c.g[A0][0] = c.g[A2][0];
        c.div_s(3, 13, 0);
        c.mov_s(4, 3);
        if c.g[S1][0] != 0 {
            // Not animated: full weight.
            c.cycles += 4;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 3);
            break 'weighted;
        }
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            // Folded at the middle.
            c.cycles += 6;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(0, 3);
            if c.fcc() {
                c.sub_s(3, 14, 3);
            } else {
                c.cycles += 2;
            }
            break 'weighted;
        }
        c.g[V1][0] = c.lb(ad(c, S0, GROUP_MIRROR));
        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            // Mirrored: slot columns - a2 - 1.
            c.cycles += 3;
            c.g[V0][0] = c.lw(ad(c, S0, GROUP_COLUMNS));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
            c.g[A0][0] = sx(ad(c, V0, 0xffff_ffff));
        }
    }
    // First wave (x).
    c.cycles += 22;
    c.lwc1(2, ad(c, GP, 0u32.wrapping_sub(0x2af8)));
    c.g[V0][0] = 0x500000;
    c.lwc1(1, ad(c, S0, 0x24));
    c.g[A1][0] = sx(ad(c, V0, 0x4fb8));
    c.mul_s(5, 4, 2);
    c.lwc1(0, ad(c, SP, 0));
    c.lwc1(6, ad(c, GP, 0u32.wrapping_sub(0x2af4)));
    c.lwc1(4, ad(c, SP, 0x10));
    c.mul_s(1, 5, 1);
    c.add_s(0, 0, 1);
    c.mul_s(0, 0, 6);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    wave_slots(c, A1);
    c.lwc1(2, ad(c, V0, 0));
    c.lwc1(1, ad(c, V1, 0));
    if c.g[S2][0] == 0 {
        c.cycles += 3;
        c.mul_s(0, 4, 3);
        c.mul_s(0, 0, 1);
        c.add_s(16, 16, 0);
    }
    // Second wave (y); the slot gets (x, running sum, 0).
    c.cycles += 30;
    c.mul_s(0, 4, 3);
    c.g[V0][0] = 0xc;
    c.mult(0, 3, 4, 2);
    c.swc1(16, ad(c, SP, 0xa4));
    c.sw(ad(c, SP, 0xa8), 0u64);
    c.mul_s(0, 0, 2);
    c.lwc1(1, ad(c, S0, 0x28));
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(ad(c, SP, 0)));
    c.swc1(0, ad(c, SP, 0xa0));
    c.mul_s(1, 5, 1);
    c.ldl(7, ad(c, SP, 0xa7));
    c.ldr(7, ad(c, SP, 0xa0));
    c.g[T0][0] = c.lw(ad(c, SP, 0xa8));
    c.sdl(7, ad(c, A0, 0x27));
    c.sdr(7, ad(c, A0, 0x20));
    c.sw(ad(c, A0, 0x28), c.g[T0][0]);
    c.lwc1(0, ad(c, SP, 0x4));
    c.add_s(0, 0, 1);
    c.mul_s(0, 0, 6);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    wave_slots(c, A1);
    c.lwc1(2, ad(c, V0, 0));
    c.lwc1(1, ad(c, V1, 0));
    if c.g[S2][0] == 0 {
        c.cycles += 4;
        c.lwc1(0, ad(c, SP, 0x14));
        c.mul_s(0, 0, 3);
        c.mul_s(0, 0, 1);
        c.add_s(15, 15, 0);
    }
    // The static shape terms (+0x34..+0x48) blended by the wind.
    c.cycles += 64;
    c.lwc1(1, ad(c, SP, 0x14));
    c.sub_s(5, 14, 12);
    c.swc1(15, ad(c, SP, 0xa4));
    c.g[A2][0] = sx(ad(c, A2, 0x1));
    c.mul_s(1, 1, 3);
    c.sw(ad(c, SP, 0xa8), 0u64);
    c.lwc1(10, ad(c, S0, 0x40));
    c.mul_s(1, 1, 2);
    c.lwc1(9, ad(c, S0, 0x44));
    c.lwc1(7, ad(c, S0, 0x48));
    c.mul_s(10, 10, 3);
    c.lwc1(8, ad(c, S0, 0x34));
    c.mul_s(9, 9, 3);
    c.lwc1(6, ad(c, S0, 0x38));
    c.mul_s(7, 7, 3);
    c.swc1(1, ad(c, SP, 0xa0));
    c.mul_s(8, 8, 3);
    c.mul_s(6, 6, 3);
    c.lwc1(0, ad(c, A0, 0x20));
    c.mul_s(10, 10, 12);
    c.lwc1(4, ad(c, S0, 0x3c));
    c.mul_s(8, 8, 5);
    c.add_s(0, 0, 1);
    c.lwc1(2, ad(c, A0, 0x24));
    c.mul_s(4, 4, 3);
    c.lwc1(3, ad(c, A0, 0x28));
    c.mul_s(9, 9, 12);
    c.swc1(0, ad(c, A0, 0x20));
    c.mul_s(7, 7, 12);
    c.mul_s(4, 4, 5);
    c.lwc1(0, ad(c, SP, 0xa4));
    c.mul_s(6, 6, 5);
    c.add_s(11, 10, 8);
    c.add_s(2, 2, 0);
    c.add_s(1, 7, 4);
    c.add_s(5, 9, 6);
    c.swc1(2, ad(c, A0, 0x24));
    c.lwc1(0, ad(c, SP, 0xa8));
    c.add_s(3, 3, 0);
    c.swc1(3, ad(c, A0, 0x28));
    c.swc1(5, ad(c, SP, 0xc4));
    c.swc1(1, ad(c, SP, 0xc8));
    c.swc1(10, ad(c, SP, 0xa0));
    c.swc1(9, ad(c, SP, 0xa4));
    c.swc1(7, ad(c, SP, 0xa8));
    c.swc1(8, ad(c, SP, 0xb0));
    c.swc1(6, ad(c, SP, 0xb4));
    c.swc1(4, ad(c, SP, 0xb8));
    c.swc1(11, ad(c, SP, 0xc0));
    c.lwc1(1, ad(c, A0, 0x20));
    c.lwc1(2, ad(c, A0, 0x24));
    c.add_s(1, 1, 11);
    c.lwc1(3, ad(c, A0, 0x28));
    c.swc1(1, ad(c, A0, 0x20));
    c.lwc1(0, ad(c, SP, 0xc4));
    c.add_s(2, 2, 0);
    c.swc1(2, ad(c, A0, 0x24));
    c.lwc1(0, ad(c, SP, 0xc8));
    c.add_s(3, 3, 0);
    c.swc1(3, ad(c, A0, 0x28));
}

/// One grid vertex (`a0` = output, `a1` = its column's offset slot, `a2` =
/// rest-vertex offset; f13 = column, f18 = row as floats; f11 = 0.5,
/// f12 = 1.0, f14 = 0.0, f15..f17 = `.lit4` constants).
pub(crate) fn grid_vertex(c: &mut Cpu) {
    'style: {
        c.cycles += 6;
        c.lwc1(0, ad(c, S0, GROUP_COLUMN_SPAN));
        c.lwc1(1, ad(c, S0, GROUP_ROW_SPAN));
        c.div_s(8, 13, 0);
        c.div_s(9, 18, 1);
        c.mov_s(10, 8);
        if c.g[S2][0] != 0 {
            c.cycles += 4;
            c.c_lt_s(11, 8);
            if c.fcc() {
                c.sub_s(8, 12, 8);
                c.cycles += 1;
            } else {
                c.cycles += 2;
            }
            break 'style;
        }
        c.cycles += 4;
        c.g[V1][0] = c.lb(ad(c, S0, GROUP_MIRROR));
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 3;
            c.sub_s(8, 12, 8);
            c.mov_s(10, 8);
            c.cycles += 1;
            break 'style;
        }
        c.cycles += 2;
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 8);
            c.cycles += 1;
        }
    }
    c.g[V0][0] = c.lb(ad(c, S0, GROUP_EDGE_ONLY));
    c.cycles += 2;
    let edge_only = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(ad(c, S0, GROUP_REST_VERTS));
    if edge_only {
        c.cycles += 3;
        c.c_eq_s(9, 14);
        if c.fcc() {
            // First row of an edge-only flag: the rest vertex as is.
            c.cycles += 8;
            c.g[V0][0] = sx(ad(c, A2, c.g[V0][0] as u32));
            c.ldl(V1, ad(c, V0, 0x7));
            c.ldr(V1, ad(c, V0, 0));
            c.g[T5][0] = c.lw(ad(c, V0, 0x8));
            c.sdl(V1, ad(c, A0, 0x7));
            c.sdr(V1, ad(c, A0, 0));
            c.sw(ad(c, A0, 0x8), c.g[T5][0]);
            return;
        }
    }
    // Rest vertex plus the column offset.
    c.cycles += 22;
    c.c_lt_s(14, 10);
    c.lwc1(5, ad(c, A1, 0));
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(ad(c, V0, 0)));
    c.lwc1(4, ad(c, A1, 0x4));
    c.lwc1(2, ad(c, V0, 0x8));
    c.lwc1(0, ad(c, V0, 0));
    c.lwc1(1, ad(c, V0, 0x4));
    c.lwc1(3, ad(c, A1, 0x8));
    c.add_s(0, 0, 5);
    c.add_s(1, 1, 4);
    c.add_s(2, 2, 3);
    c.swc1(0, ad(c, SP, 0xa0));
    c.swc1(1, ad(c, SP, 0xa4));
    c.swc1(2, ad(c, SP, 0xa8));
    c.ldl(2, ad(c, SP, 0xa7));
    c.ldr(2, ad(c, SP, 0xa0));
    c.g[V1][0] = c.lw(ad(c, SP, 0xa8));
    c.sdl(2, ad(c, A0, 0x7));
    c.sdr(2, ad(c, A0, 0));
    c.sw(ad(c, A0, 0x8), c.g[V1][0]);
    let ripple1 = c.fcc();
    c.lwc1(6, ad(c, SP, 0x18));
    if ripple1 {
        c.cycles += 4;
        c.c_lt_s(17, 6);
        let strong = c.fcc();
        c.lwc1(1, ad(c, SP, 0x8));
        if strong {
            // Ripple 1 (amplitude sp+0x18, phase sp+0x08, freq +0x2c).
            c.cycles += 37;
            c.add_s(0, 10, 9);
            c.lwc1(2, ad(c, S0, 0x2c));
            c.add_s(3, 8, 9);
            c.mul_s(2, 2, 16);
            c.lwc1(5, ad(c, A0, 0));
            c.mul_s(0, 0, 11);
            c.mul_s(3, 3, 11);
            c.mul_s(4, 6, 9);
            c.mul_s(0, 0, 2);
            c.mul_s(3, 6, 3);
            c.mul_s(2, 6, 8);
            c.add_s(1, 1, 0);
            c.mul_s(1, 1, 15);
            c.cvt_w_s(0, 1);
            c.mfc1(2, 0);
            wave_slots(c, T0);
            c.lwc1(0, ad(c, V0, 0));
            c.lwc1(1, ad(c, V1, 0));
            c.mul_s(4, 4, 0);
            c.mul_s(3, 3, 1);
            c.mul_s(2, 2, 0);
            c.swc1(4, ad(c, SP, 0xa8));
            c.swc1(3, ad(c, SP, 0xa4));
            c.swc1(2, ad(c, SP, 0xa0));
            c.add_s(5, 5, 2);
            c.lwc1(0, ad(c, A0, 0x4));
            c.lwc1(1, ad(c, A0, 0x8));
            c.add_s(0, 0, 3);
            c.swc1(5, ad(c, A0, 0));
            c.add_s(1, 1, 4);
            c.swc1(0, ad(c, A0, 0x4));
            c.swc1(1, ad(c, A0, 0x8));
        }
        c.cycles += 2;
        c.c_lt_s(14, 10);
    }
    c.cycles += 2;
    let ripple2 = c.fcc();
    c.lwc1(7, ad(c, SP, 0x1c));
    if ripple2 {
        c.cycles += 4;
        c.c_lt_s(17, 7);
        let strong = c.fcc();
        c.lwc1(1, ad(c, SP, 0xc));
        if strong {
            // Ripple 2 (amplitude sp+0x1c, phase sp+0x0c, freq +0x30).
            c.cycles += 39;
            c.add_s(0, 10, 9);
            c.lwc1(2, ad(c, S0, 0x30));
            c.add_s(4, 8, 9);
            c.mul_s(2, 2, 16);
            c.lwc1(6, ad(c, A0, 0));
            c.mul_s(0, 0, 11);
            c.mul_s(4, 4, 11);
            c.sub_s(3, 12, 9);
            c.mul_s(0, 0, 2);
            c.mul_s(5, 7, 8);
            c.mul_s(3, 7, 3);
            c.add_s(1, 1, 0);
            c.mul_s(4, 7, 4);
            c.mul_s(1, 1, 15);
            c.cvt_w_s(0, 1);
            c.mfc1(2, 0);
            wave_slots(c, T0);
            c.lwc1(0, ad(c, V0, 0));
            c.lwc1(1, ad(c, V1, 0));
            c.mul_s(5, 5, 0);
            c.mul_s(4, 4, 1);
            c.mul_s(3, 3, 0);
            c.swc1(5, ad(c, SP, 0xa0));
            c.add_s(6, 6, 5);
            c.swc1(4, ad(c, SP, 0xa4));
            c.swc1(3, ad(c, SP, 0xa8));
            c.swc1(6, ad(c, A0, 0));
            c.lwc1(1, ad(c, A0, 0x4));
            c.lwc1(2, ad(c, A0, 0x8));
            c.add_s(1, 1, 4);
            c.swc1(1, ad(c, A0, 0x4));
            c.lwc1(0, ad(c, SP, 0xa8));
            c.add_s(2, 2, 0);
            c.swc1(2, ad(c, A0, 0x8));
        }
    }
    c.cycles += 4;
    c.g[A0][0] = sx(ad(c, A0, 0xc));
    c.g[A2][0] = sx(ad(c, A2, 0xc));
    c.g[T1][0] = sx(ad(c, T1, 0xc));
    c.g[T2][0] = sx(ad(c, T2, 1));
}

// ---------------------------------------------------- cFlagManager_Update

/// `World_GetPeakNumber()` (0x2d1ba0): the current peak, 1-based.
pub(crate) const WORLD_GET_PEAK_NUMBER: u32 = 0x002d_1ba0;

/// `BXrand()`: the next random word.
pub(crate) const BX_RAND: u32 = 0x0031_77f0;

/// `cFlagManager_GroupUpdate(group)`.
pub(crate) const FLAG_MANAGER_GROUP_UPDATE: Callee = Callee {
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

// ---------------------------------------------------------------------------
// cFlagManager_Render (flagset.cpp).

/// `cFlagManager_GroupRender` (0x34b9b0).
pub(crate) const FLAG_MANAGER_GROUP_RENDER_207: Callee = Callee {
    addr: 0x0034_b9b0,
    f: ee::s11::cFlagManager_GroupRender,
};

/// `void cFlagManager_Render(cFlagManager* self)` (0x34c7f8): render each of
/// the 15 flag groups (`self+0x20`, 0x188 bytes each) that has models
/// (`group+0x5c` non-zero). s0 walks the groups, s1 counts 14 down to -1.
pub fn flag_manager_render(c: &mut Cpu) {
    const HEAD: u32 = 0x0034_c810;
    const RET: u32 = 0x0034_c824;
    const GROUPS: u32 = 0x20;
    const GROUP_SIZE: u32 = 0x188;
    /// A group's model count.
    const GROUP_MODELS: u32 = 0x5c;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(GROUPS));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xe;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).off(GROUP_MODELS).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                FLAG_MANAGER_GROUP_RENDER_207.call(c, RET)?;
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(GROUP_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFlagManager_GroupSave (0x34c2e0).

/// `cFlagGroup` field: `int` item count.
pub(crate) const FLAG_GROUP_COUNT: u32 = 0x5c;

/// `cFlagGroup` field: `cFlag*[]` items.
pub(crate) const FLAG_GROUP_ITEMS: u32 = 0x94;

/// Stream vtable slot: `Write(this, const void* data, int size)`.
pub(crate) const FLAG_STREAM_WRITE: u32 = 0x8;

/// Stream vtable slot: `WriteObject(this, void* object)`.
pub(crate) const FLAG_STREAM_WRITE_OBJECT: u32 = 0x28;

/// `void cFlagManager_GroupSave(cFlagGroup* group, Stream* stream)`
/// (0x34c2e0): write the group's 0x60-byte header to the stream, then each
/// of its items (`count` at +0x5c, pointers from +0x94) through the
/// stream's object writer.
///
/// Frame 0x50 (s0 +0x40, s1 +0x30, s2 +0x20, s3 +0x10, ra +0).
pub fn cflag_manager_group_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        set_reg(c, A2, 0x60);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        set_reg(c, S1, 0);
        frame.save(c, S3, 0x10);
        let stream = reg(c, A1);
        set_reg(c, S2, stream);
        frame.save(c, S0, 0x40);
        let group = reg(c, A0);
        set_reg(c, S3, group);
        frame.save_ra(c, 0);
        set_reg(c, A1, group);
        let vt = Addr(stream as u32).lw(c);
        set_reg(c, V0, vt);
        let write = VSlot::read(c, vt, FLAG_STREAM_WRITE);
        set_reg(c, A0, write.delta);
        set_reg(c, V1, write.func);
        set_reg(c, A0, write.this(stream));
        call_ptr(c, write.func as u32, 0x0034_c320)?;
        let group = reg(c, S3);
        let count = Addr(group as u32).off(FLAG_GROUP_COUNT).lw(c);
        set_reg(c, V0, count);
        set_reg(c, S0, addu(group, FLAG_GROUP_ITEMS as u64));
        if (count as i64) > 0 {
            c.cycles += 1;
            let vt = Addr(r32(c, S2)).lw(c);
            set_reg(c, V0, vt);
            loop {
                // Loop head 0x34c330 (s0 the item pointer, s1 the items done,
                // v0 the stream's vtable).
                c.cycles += 11;
                let done = addu(reg(c, S1), 1);
                set_reg(c, S1, done);
                let item_at = reg(c, S0);
                let item = Addr(item_at as u32).lw(c);
                set_reg(c, A1, item);
                let vt = reg(c, V0);
                let write = VSlot::read(c, vt, FLAG_STREAM_WRITE_OBJECT);
                set_reg(c, S0, addu(item_at, 4));
                set_reg(c, V1, write.func);
                let stream = reg(c, S2);
                set_reg(c, A0, write.this(stream));
                call_ptr(c, write.func as u32, 0x0034_c34c)?;
                let count = Addr(r32(c, S3)).off(FLAG_GROUP_COUNT).lw(c);
                let more = ((reg(c, S1) as i64) < (count as i64)) as u64;
                set_reg(c, V0, more);
                if more == 0 {
                    break;
                }
                let vt = Addr(r32(c, S2)).lw(c);
                set_reg(c, V0, vt);
                if event_due(c) {
                    deliver_event_here(c, 0x0034_c330);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimObject_BaseDtor (0x34dba8).

/// The base vtable this destructor installs (`+0x20`).
pub(crate) const ANIM_OBJ_BASE_VTABLE: u32 = 0x0049_0cc8;

pub(crate) const ANIM_OBJ_VTABLE: u32 = 0x20;

/// `+0x14` the embedded ModObj; `+0x44` a cache block, `+0x48` the item
/// array (`new[]`: count at `-0x10`, 0xd0-byte items with their vtable at
/// `+0x84`), `+0x4c` a buffer.
pub(crate) const ANIM_OBJ_MOD_OBJ: u32 = 0x14;

pub(crate) const ANIM_OBJ_CACHE: u32 = 0x44;

pub(crate) const ANIM_OBJ_ITEMS: u32 = 0x48;

pub(crate) const ANIM_OBJ_BUF: u32 = 0x4c;

pub(crate) const ANIM_ITEM_SIZE: u32 = 0xd0;

pub(crate) const ANIM_ITEM_VTABLE: u32 = 0x84;

/// `EA_MemFree(p)` and `operator delete(p)` (through the table).
pub(crate) const ANIM_EA_MEM_FREE: u32 = 0x0031_7e98;

pub(crate) const ANIM_OPERATOR_DELETE: u32 = 0x0031_7e50;

pub(crate) const ANIM_MOD_OBJ_DTOR: Callee = Callee {
    addr: 0x0035_53c0,
    f: ee::s11::ModObj_dtor,
};

/// `void AnimObject_BaseDtor(AnimObject* self, int flags)` (0x34dba8):
/// reset the vtable, free the cache, destroy the item array back to front
/// (virtual destructor slot 1 of each, flags 0) and free it, free the
/// buffer, destroy the embedded ModObj, and `delete self` when `flags & 1`.
pub fn anim_object_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = sx(ANIM_OBJ_BASE_VTABLE);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let me = Addr(r32(c, S1));
        me.off(ANIM_OBJ_VTABLE).sw(c, c.g[V0][0]);
        c.g[A0][0] = me.off(ANIM_OBJ_CACHE).lw(c);
        c.g[S0][0] = c.g[A1][0];
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, ANIM_EA_MEM_FREE, 0x0034_dbe4)?;
        }

        c.cycles += 4;
        let me = Addr(r32(c, S1));
        c.g[A1][0] = me.off(ANIM_OBJ_ITEMS).lw(c);
        c.g[S3][0] = c.g[S0][0] & 1;
        c.g[S2][0] = sx(me.0.wrapping_add(ANIM_OBJ_MOD_OBJ));
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            let items = r32(c, A1);
            c.g[V1][0] = Addr(items.wrapping_sub(0x10)).lw(c);
            c.g[V0][0] = ANIM_ITEM_SIZE as u64;
            c.mult(0, A0, V1, V0);
            c.g[S0][0] = sx(r32(c, A0).wrapping_add(items));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = me.off(ANIM_OBJ_ITEMS).lw(c);
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(ANIM_ITEM_SIZE));
                loop {
                    // Item s0's virtual destructor (slot 1), flags 0.
                    c.cycles += 9;
                    let item = r32(c, S0);
                    c.g[A1][0] = 0;
                    c.g[V0][0] = Addr(item.wrapping_add(ANIM_ITEM_VTABLE)).lw(c);
                    let vt = r32(c, V0);
                    c.g[A0][0] = Addr(vt.wrapping_add(8)).lh(c);
                    c.g[V1][0] = Addr(vt.wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = sx(item.wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0034_dc28)?;
                    let items = Addr(r32(c, S1)).off(ANIM_OBJ_ITEMS).lw(c);
                    c.g[V0][0] = items;
                    if items != c.g[S0][0] {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(ANIM_ITEM_SIZE));
                        if event_due(c) {
                            deliver_event_here(c, 0x0034_dc10);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = Addr(r32(c, S1)).off(ANIM_OBJ_ITEMS).lw(c);
                    break;
                }
            }
            // Free the block (the array starts 0x10 bytes into it).
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
            call_ptr(c, ANIM_EA_MEM_FREE, 0x0034_dc40)?;
        }

        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, S1)).off(ANIM_OBJ_BUF).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, ANIM_EA_MEM_FREE, 0x0034_dc54)?;
        }
        c.g[A0][0] = c.g[S2][0];

        c.cycles += 4;
        c.g[A1][0] = 0;
        ANIM_MOD_OBJ_DTOR.call(c, 0x0034_dc60)?;
        let delete = c.g[S3][0] != 0;
        frame.restore(c, S0, 0x40);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, ANIM_OPERATOR_DELETE, 0x0034_dc70)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFlagManager`: 15 flag groups of 0x188 bytes from +0x20; a group's
/// item count is at +0x5c.
pub(crate) const FLAG_MANAGER_GROUPS: u32 = 0x20;

pub(crate) const FLAG_MANAGER_GROUP_SIZE: u32 = 0x188;

#[cfg(test)]
pub(crate) const FLAG_MANAGER_GROUP_COUNT: u32 = 0x5c;

/// `cFlagManager_GroupSave(group, stream)` (0x34c2e0).
pub(crate) const FLAG_MANAGER_GROUP_SAVE: Callee = Callee {
    addr: 0x0034_c2e0,
    f: ee::s11::f_0034c2e0,
};

/// `void cFlagManager_Save(cFlagManager *self, Stream *stream)` (0x34c848):
/// save the 15 flag groups that follow the header.
pub fn c_flag_manager_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(FLAG_MANAGER_GROUPS));
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0xe;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            FLAG_MANAGER_GROUP_SAVE.call(c, 0x0034_c874)?;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(FLAG_MANAGER_GROUP_SIZE));
            if !more {
                c.cycles += 6;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_c868);
            }
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const FLAG_MANAGER_GROUP_COUNT_2: u32 = 15;

pub(crate) const FLAG_GROUP_SIZE: u32 = 0x188;

pub(crate) const FLAG_GROUP_MEMBERS: u32 = 0x5c;

pub(crate) const C_FLAG_MANAGER_GROUP_REMOVE_FLAG: Callee = Callee {
    addr: 0x0034_b168,
    f: ee::s11::cFlagManager_GroupRemoveFlag,
};

/// `void cFlagManager_RemoveFlag(cFlagManager* self, Flag* flag)`
/// (0x34c600): offer the flag to each used group in turn
/// (`cFlagManager_GroupRemoveFlag`) until one removes it.
pub fn c_flag_manager_remove_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(FLAG_MANAGER_GROUPS));
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        loop {
            c.cycles += 3;
            let group = r32(c, S0);
            c.g[V0][0] = c.lw(group.wrapping_add(FLAG_GROUP_MEMBERS));
            c.g[A0][0] = c.g[S0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                C_FLAG_MANAGER_GROUP_REMOVE_FLAG.call(c, 0x0034_c634)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break;
                }
            }
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            let more = (c.g[S1][0] as i64) < FLAG_MANAGER_GROUP_COUNT_2 as i64;
            c.g[V0][0] = more as u64;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(FLAG_GROUP_SIZE));
            if !more {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_c620);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimObject_BaseCtor (0x34d9b0)

/// AnimObject: current and start time (+0, +4), window start / end
/// (+8, +0xc, "never" = 1e10), the active flag (+0x10), the embedded
/// ModObj (+0x14; its vtable at +0x20, its node at +0x2c), the node-slot
/// array (+0x44) and the mesh instances (+0x48).
pub(crate) const ANIM_OBJECT_BASE_CTOR_MOD: u32 = 0x14;

pub(crate) const ANIM_OBJECT_BASE_CTOR_SLOTS: u32 = 0x44;

pub(crate) const ANIM_OBJECT_BASE_CTOR_MESHES: u32 = 0x48;

/// The AnimObject ModObj vtable (`D_00490CC8`) and the slot array's
/// allocation name (`D_0048E758`).
pub(crate) const ANIM_OBJECT_BASE_CTOR_VTABLE: u32 = 0x0049_0cc8;

pub(crate) const ANIM_OBJECT_BASE_CTOR_SLOT_NAME: u32 = 0x0048_e758;

/// gp-relative 1e10 ("no window").
pub(crate) const ANIM_OBJECT_BASE_CTOR_NEVER_GP: u32 = 0x2aa8;

/// `AnimObject* AnimObject_BaseCtor(AnimObject* self, int list, int kind,
/// Node* node, int force)` (0x34d9b0): constructs the embedded ModObj on
/// `node`, sets the AnimObject vtable, allocates one 64-byte slot per node
/// definition, builds the mesh instances (`AnimObject_AllocNodeInfo`),
/// starts at time 0 with the duration of the first mesh, an open window
/// (1e10), active; when `force` or the node's flag bit 0 is set, the node
/// flags get bit 2 instead of bit 1. Finally evaluates the meshes
/// (`AnimObject_v52`). Returns `self`.
pub fn anim_object_base_ctor(c: &mut Cpu) {
    const MOD_OBJ_CTOR: Callee = Callee {
        addr: 0x0035_5280,
        f: ee::s11::ModObj_ctor,
    };
    const ALLOC_NODE_INFO: Callee = Callee {
        addr: 0x0034_e448,
        f: ee::s11::AnimObject_AllocNodeInfo,
    };
    const GET_DURATION: Callee = Callee {
        addr: 0x0035_1508,
        f: ee::s11::IntersectInfo_GetDuration,
    };
    const V52: Callee = Callee {
        addr: 0x0034_e348,
        f: ee::s11::AnimObject_v52,
    };
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(ANIM_OBJECT_BASE_CTOR_MOD));
        frame.save(c, S2, 0x20);
        c.g[A0][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T0][0]; // force
        MOD_OBJ_CTOR.call(c, 0x0034_d9dc)?;

        // The node-slot array: new (name, 0x20000000, 0) Slot[definitions].
        let me = r32(c, S1);
        c.g[S2][0] = sx(me.wrapping_add(ANIM_OBJECT_BASE_CTOR_SLOTS));
        c.g[V1][0] = sx(ANIM_OBJECT_BASE_CTOR_VTABLE);
        c.g[A1][0] = sx(ANIM_OBJECT_BASE_CTOR_SLOT_NAME);
        Addr(me).off(0x20).sw(c, sx(ANIM_OBJECT_BASE_CTOR_VTABLE));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(r32(c, S0)).off(0x18).lw(c); // node
        c.g[V1][0] = Addr(r32(c, V0)).off(0x80).lw(c); // node definitions
        c.g[S0][0] = Addr(r32(c, V1)).off(4).lw(c); // definition count
        c.g[A0][0] = sx(r32(c, S0) << 6);
        call_ptr(c, 0x0031_7e30, 0x0034_da10)?; // operator_new_named
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        let slots = c.g[V0][0];
        Addr(r32(c, S2)).sw(c, slots);
        ALLOC_NODE_INFO.call(c, 0x0034_da2c)?;

        // Start time: the first mesh's duration, else 0.
        c.g[A0][0] = Addr(r32(c, S1)).off(ANIM_OBJECT_BASE_CTOR_MESHES).lw(c);
        c.f[1] = 0;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            GET_DURATION.call(c, 0x0034_da44)?;
            c.f[1] = c.f[0];
        }

        c.cycles += 9;
        let me = r32(c, S1);
        let gp = r32(c, GP);
        c.f[0] = Addr(gp.wrapping_sub(ANIM_OBJECT_BASE_CTOR_NEVER_GP))
            .read_fpu(c)
            .0;
        c.g[V0][0] = 1;
        let (start, never) = (c.f[1], c.f[0]);
        Addr(me).off(4).sw(c, start as u64);
        c.g[A0][0] = sx(me.wrapping_add(ANIM_OBJECT_BASE_CTOR_MOD));
        Addr(me).off(0x10).sw(c, 1);
        Addr(me).off(0xc).sw(c, never as u64);
        Addr(me).sw(c, start as u64);
        Addr(me).off(8).sw(c, never as u64);
        let set = if c.g[S3][0] != 0 {
            true
        } else {
            c.cycles += 5;
            c.g[V1][0] = Addr(r32(c, A0)).off(0x18).lw(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(8).lw(c) & 1;
            c.g[V0][0] != 0
        };
        if set {
            // Node flags: clear bit 1, set bit 2.
            c.cycles += 7;
            c.g[A0][0] = Addr(r32(c, A0)).off(0x18).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_fffd;
            let flags = (Addr(r32(c, A0)).off(8).lw(c) & !2) | 4;
            c.g[V0][0] = flags;
            Addr(r32(c, A0)).off(8).sw(c, flags);
        }

        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        V52.call(c, 0x0034_daa4)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;

    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::input::inputmap::tests::put_f32;
    pub(crate) use crate::game::intersect::worldsphtree::tests::f;

    pub(crate) use crate::game::intersect::worldsphtree::tests::save_test;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;

    pub(crate) use crate::game::util::menu::tests::w as w_5;
    pub(crate) use crate::game::util::menu::tests::wf_2 as wf;

    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_ctx;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::uf;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::BASE;

    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_3 as SEEDS_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::w as w_3;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_4;

    use ssx3_ee as ee;

    #[test]
    fn fence_add_impulse_matches() {
        check(
            ssx3_ee::s11::f_0034d6f0,
            fence_add_impulse,
            300,
            &|c, seed| {
                let mut r = Rng(0x1015 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[GP][0] = GAME_GP as u64;
                c.f[12] = r.edge_f32();
                c.bus.write32(DATA + 0x30, r.edge_f32());
                c.bus.write32(DATA + 0x34, r.edge_f32());
                c.bus.write32(GAME_GP - 0x2ab4, r.edge_f32());
                c.bus.write32(GAME_GP - 0x2ab0, r.edge_f32());
                c.bus.write32(GAME_GP - 0x2aac, r.edge_f32());
            },
        );
    }

    #[test]
    fn anim_object_get_node_intersect_matrix_matches() {
        check_lohi(
            ssx3_ee::s11::f_0034e320,
            anim_object_get_node_intersect_matrix,
            200,
            &|c, seed| {
                let mut r = Rng(0x1016 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.bus
                    .write32(DATA + 0x48, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
                c.bus.write32(DATA + 0x100 + 0x90, r.next_u32());
            },
        );
    }

    #[test]
    fn anim_object_build_matrices_matches() {
        const SELF: u32 = DATA + 0x1000;
        const MODEL: u32 = DATA + 0x1100;
        const SKEL: u32 = DATA + 0x1200;
        const BONES: u32 = DATA + 0x1400;
        const OUT: u32 = DATA + 0x2000;
        const ROOT: u32 = DATA + 0x3000;
        const NODES: u32 = DATA + 0x4000;
        const OWN: u32 = DATA + 0x6000;
        check(
            ssx3_ee::s11::f_0034dd18__generated,
            anim_object_build_matrices,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4005, 0);
                let n = 1 + seed % 4;
                c.bus.write32(SELF + 0x2c, MODEL);
                c.bus.write32(SELF + 0x48, NODES);
                c.bus.write32(MODEL + 0x80, SKEL);
                c.bus.write32(MODEL + 0x84, f(1.0));
                c.bus.write32(SKEL + 4, n);
                c.bus.write32(SKEL + 8, BONES);
                for i in 0..n {
                    let b = BONES + 0x10 * i;
                    let parent = if i == 0 || seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        i - 1
                    };
                    c.bus.write32(b, parent);
                    c.bus.write32(b + 8, (seed + i) % 2);
                    c.bus.write32(b + 0xc, OWN + 0x40 * i);
                    for k in 0..16 {
                        c.bus.write32(OWN + 0x40 * i + 4 * k, r.f32(2.0));
                    }
                }
                for k in 0..n {
                    let m = NODES + 0x90 + 0xd0 * k;
                    for j in 0..16 {
                        c.bus.write32(m + 4 * j, r.f32(1.0));
                    }
                }
                for k in 0..16 {
                    c.bus.write32(ROOT + 4 * k, r.f32(1.0));
                }
                for k in 0..(16 * n) {
                    c.bus.write32(OUT + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(ROOT);
                c.g[A2][0] = sx(OUT);
            },
        );
    }

    #[test]
    fn flag_manager_group_render_matches() {
        check(
            ee::s11::f_0034b9b0__generated,
            flag_manager_group_render,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb9b0, 5);
                put_ctx(c, &mut r, &[0x158, 0x2e8, 0x300]);
                c.bus.write32(
                    GAME_GP.wrapping_add(globals::FLAG_MANAGER as u32),
                    (seed % 8 != 0) as u32,
                );
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
                c.bus
                    .write32(GAME_GP.wrapping_add(globals::FLAG_TABLE as u32), table);
                c.bus.write32(table, mbase);
                for idx in 0..4u32 {
                    c.bus
                        .write32(mbase + 8 * idx + 0x24, if idx == 1 { 0 } else { 6 });
                }
                for idx in 0..4u32 {
                    c.bus
                        .write32(mbase + 0x3f0 + 0x18 * idx, if idx < 2 { 3 } else { 0 });
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
                    c.bus
                        .write32(LOG_RESULTS + 4 * k, if seed % 2 == 0 { 0 } else { 1 });
                }
                c.g[A0][0] = sx(group);
            },
        );
    }

    #[test]
    fn cflag_manager_group_update_matches() {
        covered(
            ssx3_ee::s11::f_0034b818__generated,
            cflag_manager_group_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34b818, 1);
                let selfp = DATA;
                let world = DATA + 0x4000;
                let w84 = DATA + 0x4100;
                let mesh = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_add(0x1150), 1);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, w84);
                c.bus.write32(w84 + 0x70, DATA + 0x4200);
                c.bus.write32(w84 + 0xc, DATA + 0x4300);
                let frame = r.next_u32() % 8;
                c.bus.write32(DATA + 0x4308, frame);
                c.g[A0][0] = selfp as u64;
                c.bus
                    .write32(selfp + 0x5c, if seed % 7 == 0 { 0 } else { 1 });
                c.bus
                    .write32(selfp + 0x60, if seed % 5 == 0 { 0 } else { mesh });
                c.bus.write32(
                    selfp + 0x90,
                    if seed % 2 == 0 {
                        frame & 1
                    } else {
                        (frame + 1) & 1
                    },
                );
                put_f32(c, selfp + 0x4c, r.edge_f32());
                put_f32(c, DATA + 0x4200 + 0x10, r.edge_f32());
                for i in 0..4 {
                    put_f32(c, selfp + 4 + 4 * i, r.edge_f32());
                    put_f32(c, selfp + 0x78 + 4 * i, r.edge_f32());
                }
                put_f32(c, selfp + 0x50, if seed % 4 == 0 { 0 } else { r.f32(1.0) });
                put_f32(c, selfp + 0x54, if seed % 4 == 1 { 0 } else { r.f32(1.0) });
                put_f32(c, selfp + 0x88, r.f32(1.0));
                put_f32(c, selfp + 0x8c, r.f32(1.0));
                put_f32(c, GAME_GP.wrapping_sub(0x2b00), 0x3a83_126f); // 0.001f
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(mesh + 4, vt);
            },
        );
    }

    /// The flag's wave phases (`+0x78..`) and texture scroll (`+0x88`,
    /// `+0x8c`) follow the rate once: Flag_ReadParams turns the authored
    /// per-second amounts into per-tick ones at the running rate, and the
    /// update adds them as they are. A 60 Hz tick of game time (one update
    /// at 60 Hz, `rate/60` at 120 and 240 Hz) ends at the same values.
    #[test]
    fn cflag_manager_group_update_follows_the_rate() {
        fn after(hz: u32) -> Vec<f32> {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let _ = machine(c, 1, 0x34b819, 0);
            crate::tick::set_for_test(c, hz);
            let selfp = DATA;
            let src = DATA + 0x5000;
            let world = DATA + 0x4000;
            let w84 = DATA + 0x4100;
            c.bus.write32(GAME_GP.wrapping_add(0x1150), 1);
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(world + 0x84, w84);
            c.bus.write32(w84 + 0x70, DATA + 0x4200);
            c.bus.write32(w84 + 0xc, DATA + 0x4300);
            c.bus.write32(DATA + 0x4308, 2);
            // Authored per second: amounts (src +0x10, +0x1c, +0x28, +0x34),
            // blend (+0x58), scroll v and u (+0x5c, +0x60). The group's
            // first flag copies the descriptor, so read it in place.
            for i in 0..4u32 {
                put_f32(c, src + 0x10 + 0xc * i, f(0.6 * (i + 1) as f32));
            }
            put_f32(c, src + 0x58, f(0.25));
            put_f32(c, src + 0x5c, f(1.2));
            put_f32(c, src + 0x60, f(1.8));
            c.g[GP][0] = GAME_GP as u64;
            c.g[RA][0] = crate::difftest::RA as u64;
            c.g[A0][0] = selfp as u64;
            c.g[A1][0] = src as u64;
            crate::game::object::flexrailnode::flag_read_params(c);
            c.bus.write32(selfp + 0x5c, 1);
            c.bus.write32(selfp + 0x60, DATA + 0x2000);
            // Parity never matches the (fixed) frame count: no rebuild.
            c.bus.write32(selfp + 0x90, 1);
            put_f32(c, DATA + 0x4200 + 0x10, f(0.5));
            for i in 0..4 {
                put_f32(c, selfp + 0x78 + 4 * i, f(0.1));
            }
            put_f32(c, selfp + 0x88, f(0.2));
            put_f32(c, selfp + 0x8c, f(0.3));
            put_f32(c, GAME_GP.wrapping_sub(0x2b00), 0x3a83_126f); // 0.001f
            for _ in 0..hz / 60 {
                c.g[A0][0] = selfp as u64;
                cflag_manager_group_update(c);
                assert_eq!(c.pc, crate::difftest::RA);
            }
            let mut out: Vec<f32> = (0..4)
                .map(|i| f32::from_bits(c.bus.read32(selfp + 0x78 + 4 * i)))
                .collect();
            out.push(f32::from_bits(c.bus.read32(selfp + 0x88)));
            out.push(f32::from_bits(c.bus.read32(selfp + 0x8c)));
            out
        }
        let base = after(60);
        assert!((base[0] - 0.1).abs() > 1e-4 && (base[4] - 0.2).abs() > 1e-4);
        for hz in [120, 240] {
            for (a, b) in base.iter().zip(after(hz)) {
                assert!((a - b).abs() < 1e-5, "{hz} Hz: {b} vs {a} at 60");
            }
        }
    }

    #[test]
    fn anim_object_v52_matches() {
        covered(
            ssx3_ee::s11::f_0034e348__generated,
            anim_object_v52,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34e348, 3);
                let selfp = DATA;
                c.g[A0][0] = selfp as u64;
                let items = DATA + 0x1000;
                let count = seed % 4;
                c.bus.write32(selfp + 0x40, count);
                c.bus
                    .write32(selfp + 0x48, if count == 0 { 0 } else { items });
                put_f32(c, selfp + 4, r.edge_f32());
                c.bus.write32(selfp + 0x10, r.next_u32());
                let vt = vtable(c, &mut r, &[0x10]);
                for i in 0..3 {
                    c.bus.write32(items + 0xd0 * i + 0x84, vt);
                }
            },
        );
    }

    #[test]
    fn fence_save_matches() {
        save_test(ee::s11::f_0034d960__generated, fence_save, 0x34d9_60, 0x1c);
    }

    #[test]
    fn c_flag_manager_load_matches() {
        check(
            ee::s11::f_0034c898__generated,
            c_flag_manager_load,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34c8_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x2000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn fence_kick_from_neighbour_matches() {
        check(
            ee::s11::f_0034cde8__generated,
            fence_kick_from_neighbour,
            SEEDS_5,
            &|c, seed| {
                let _ = base(c, seed, 0x34cd_e8);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                w(c, s + 0x38, if seed % 3 == 0 { 0 } else { 1 });
                w(c, s + 0x3c, if seed % 4 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn fence_dtor_matches() {
        check(
            ee::s11::f_0034cb80__generated,
            fence_dtor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_cb80, 1);
                c.g[A0][0] = sx(DATA);
                // Even flags: `WorldObj_dtor` passes them on to a process
                // destructor that would otherwise free the test object.
                c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
                c.bus
                    .write32(DATA + 0x40, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
                // The world interface (gp_3+0x2a90), its table at +0x10d8 and slot 77.
                let vt = DATA + 0x300;
                c.bus.write16(vt + 0x268, 0);
                c.bus.write32(vt + 0x26c, FAKE);
                c.bus.write32(DATA + 0x400 + 0x10d8, vt);
                c.bus.write32(GAME_GP + 0x2a90, DATA + 0x400);
                c.bus.write32(DATA + 0x18, DATA + 0x500);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn c_flag_manager_ctor_matches() {
        check(
            ee::s11::f_0034c428__generated,
            c_flag_manager_ctor,
            40,
            &|c, seed| {
                let _ = base_2(c, seed, 0x34_c428);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(seed);
            },
        );
    }

    #[test]
    fn c_flag_manager_dtor_matches() {
        check(
            ee::s11::f_0034c4b8__generated,
            c_flag_manager_dtor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x34c4, 0);
                env(c);
                let this = DATA + 0x400;
                let vt = DATA + 0x2000;
                slot(c, vt, 8, EMPTY);
                for i in 0..15u32 {
                    w32(c, this + 0x20 + 0x188 * i + 0x184, vt);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    // ----------------------------------------------------------------- fence

    #[test]
    fn fence_v40_matches() {
        check(
            ssx3_ee::s11::f_0034d650__generated,
            fence_v40,
            12,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x0034_d650);
                let fence = DATA + 0x100;
                let impulse = DATA + 0x2000;
                clear_2(c, fence, 0x100);
                for k in 0..16 {
                    w_3(c, impulse + 4 * k, r.edge_f32());
                }
                w_3(
                    c,
                    fence + 0x30,
                    if seed % 3 == 0 {
                        1.5f32.to_bits()
                    } else {
                        0.25f32.to_bits()
                    },
                );
                w_3(c, fence + 0x38, if seed % 2 == 0 { 0 } else { 4 });
                w_3(c, fence + 0x3c, if seed % 2 == 0 { 0 } else { 4 });
                let cst = gp(c, -0x2ab8);
                c.bus.write32(cst, r.edge_f32());
                c.g[A0][0] = sx(fence);
                c.g[A1][0] = sx(impulse);
            },
        );
    }

    #[test]
    fn flag_manager_group_load_matches() {
        check(
            ee::s11::f_0034c378__generated,
            cflag_manager_group_load,
            40,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x34c3);
                let stream = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(stream);
                c.bus.write32(stream, DATA + 0x500);
                c.bus.write16(DATA + 0x500 + 0x10, 0);
                c.bus.write32(DATA + 0x500 + 0x14, FAKE);
                c.bus.write16(DATA + 0x500 + 0x18, 0);
                c.bus.write32(DATA + 0x500 + 0x1c, FAKE + 0x10);
                c.bus.write32(DATA + 0x5c, (seed % 2) as u32);
                fill(c, &mut r, DATA + 0x94, 8);
            },
        );
    }

    // ---------------------------------------------------------------- engine

    #[test]
    fn cflag_manager_add_flag_matches() {
        check(
            ee::s11::f_0034c548__generated,
            cflag_manager_add_flag,
            6,
            &|c, seed| {
                let _ = base_5(c, seed, 0x0034_c548, 2);
                c.pc = 0x0034_c548;
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 7) as u64;
                let used = (seed % 4) as u32;
                for i in 0..15u32 {
                    let slot = OBJ + 0x20 + i * 0x188;
                    w_4(c, slot + 0x5c, if i < used { 1 } else { 0 });
                    w_4(c, slot + 4, (seed % 4) as u32);
                }
            },
        );
    }

    // -- 0x0031bcb0 Quat_Slerp ------------------------------------------

    // -- 0x0034d778 Fence_Render ---------------------------------------

    #[test]
    fn fence_render_matches() {
        covered(
            ssx3_ee::s11::f_0034d778__generated,
            fence_render,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34d778, 4);
                let world = DATA + 0x8_0000;
                let fence = DATA + 0x1000;
                let board = DATA + 0x2000;
                let manager = DATA + 0x3000;
                let obj = DATA + 0x4000;
                let anim = DATA + 0x5000;
                let draw = DATA + 0x6000;
                w_5(c, GAME_GP.wrapping_add(0x16c8), manager);
                w_5(c, GAME_GP.wrapping_add(0x2a90), world);
                w_5(c, fence + 0x18, board);
                w_5(c, fence + 0x40, anim);
                c.g[A0][0] = sx(fence);
                c.bus
                    .write8(board + 0x7d, if seed % 3 == 0 { 0xff } else { 1 });
                c.bus.write16(board + 0x7e, (r.next_u32() & 3) as u16);
                let table = DATA + 0x7000;
                w_5(c, manager, table);
                for i in 0..4u32 {
                    w_5(c, table + 8 * i + 0x24, 6);
                    w_5(c, table + 0x18 * i + 0x3f0, 3);
                }
                w_5(c, anim + 4, obj);
                w_5(c, anim + 0x38, 0);
                w_5(c, anim + 0x3c, FAKE);
                w_5(c, anim + 0x94, 0);
                w_5(c, world + 0xe84, draw);
                w_5(c, world + 0x10d8, obj);
                w_5(c, obj + 0x2e8, 0);
                w_5(c, obj + 0x158, 0);
                w_5(c, obj + 0x15c, FAKE + 0x20);
                w_5(c, obj + 0x2ec, FAKE + 0x30);
                w_5(c, obj + 0x300, 0);
                w_5(c, obj + 0x304, FAKE + 0x40);
                for off in [0x60u32, 0x64, 0x68, 0x6c, 0x70, 0x74] {
                    wf(c, board + off, r.f32(10.0));
                }
                for i in 0..0x30u32 {
                    c.bus.write8(draw + i, i as u8 | 1);
                }
                w_5(c, draw + 0x18, r.next_u32());
            },
        );
    }

    // ---------------------------------------------------------- Fence_Update

    /// A hit fence at `DATA + 0x100` (no mesh, `+0x40 = 0`, so the real
    /// `Fence_PropagateToNeighbour` / `Fence_UpdateVerts` take their short
    /// paths), its interface (`+0xc`, sleep slot `+0x110` a fake) and the
    /// three constants it reads from `gp`.
    pub(crate) fn fence_update_fixture(c: &mut Cpu, r: &mut Rng) -> u32 {
        let fence = DATA + 0x100;
        for k in 0..0x48u32 {
            c.bus.write32(fence + 4 * k, 0);
        }
        let vt = vtable(c, r, &[0x110]);
        c.bus.write32(fence + 0xc, vt);
        c.bus.write32(fence + 0x104, ptr(r));
        c.bus.write32(fence + 0x108, ptr(r));
        c.bus
            .write32(GAME_GP.wrapping_sub(0x2ac8), 0.01f32.to_bits());
        // The executable's values: both bounce factors are +0.1 (the
        // velocity turns round through the `neg.s` before the multiply).
        c.bus
            .write32(GAME_GP.wrapping_sub(0x2ac4), 0.1f32.to_bits());
        c.bus
            .write32(GAME_GP.wrapping_sub(0x2ac0), 0.1f32.to_bits());
        c.g[A0][0] = sx(fence);
        fence
    }

    #[test]
    fn fence_update_matches() {
        covered(
            ee::s11::f_0034ce48__generated,
            fence_update,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34ce48, 1);
                let fence = fence_update_fixture(c, &mut r);
                // Neighbour delays: idle, running out this tick, still running,
                // negative.
                let delay = |k: u32| match k % 4 {
                    0 => 0u32,
                    1 => 1,
                    2 => 3,
                    _ => 0xffff_fffe,
                };
                c.bus.write32(fence + 0x38, delay(seed));
                c.bus.write32(fence + 0x3c, delay(seed / 4));
                // Sway: settled (sleeps, two cases), about to bounce up or down,
                // mid-swing, random.
                let (angle, amp, vel) = match seed % 6 {
                    0 => (0.004f32, 0.008f32, 0.0005f32),
                    1 => (0.49, 0.5, 0.02),
                    2 => (-0.49, 0.5, -0.02),
                    3 => (0.1, 0.5, 0.02),
                    4 => (0.008, 0.005, -0.003),
                    _ => (
                        f32::from_bits(r.f32(1.0)),
                        f32::from_bits(r.f32(1.0)),
                        f32::from_bits(r.f32(0.1)),
                    ),
                };
                c.bus.write32(fence + 0x2c, angle.to_bits());
                c.bus.write32(fence + 0x30, amp.to_bits());
                c.bus.write32(fence + 0x34, vel.to_bits());
            },
        );
    }

    /// Run `Fence_Update` once at the test machine's rate.
    fn fence_update_tick(c: &mut Cpu, fence: u32) {
        c.g[A0][0] = sx(fence);
        c.g[RA][0] = sx(crate::difftest::RA);
        fence_update(c);
        assert_eq!(c.pc, crate::difftest::RA);
    }

    /// The neighbour delay runs out after 4 frames at 60 Hz at any rate
    /// (4, 8 and 16 updates), and one 60 Hz frame of updates moves the sway
    /// angle by one frame's velocity.
    #[test]
    fn fence_update_follows_the_rate() {
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 1, 0x34ce48, 0);
            let fence = fence_update_fixture(c, &mut r);
            crate::tick::set_for_test(c, hz);
            let delay = fence_kick_delay(c) as u32;
            c.bus.write32(fence + 0x38, delay);
            c.bus.write32(fence + 0x2c, 0.1f32.to_bits());
            c.bus.write32(fence + 0x30, 0.5f32.to_bits());
            c.bus.write32(fence + 0x34, 0.02f32.to_bits());
            let mut ticks = 0;
            while c.bus.read32(fence + 0x38) != 0 {
                fence_update_tick(c, fence);
                ticks += 1;
                if ticks == hz / 60 {
                    let angle = f32::from_bits(c.bus.read32(fence + 0x2c));
                    assert!((angle - 0.12).abs() < 1e-5, "{hz} Hz: angle {angle}");
                }
            }
            assert_eq!(ticks, 4 * hz / 60, "{hz} Hz");
        }
    }

    /// A hit fence wobbles for the same time at any rate: the number of
    /// updates until it goes to sleep, in 60 Hz frames, is within a few
    /// frames of the 60 Hz count.
    #[test]
    fn fence_update_settles_in_the_same_time() {
        fn frames(hz: u32) -> f32 {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 2, 0x34ce48, 0);
            let fence = fence_update_fixture(c, &mut r);
            crate::tick::set_for_test(c, hz);
            c.bus.write32(fence + 0x2c, 0.0f32.to_bits());
            c.bus.write32(fence + 0x30, 0.5f32.to_bits());
            c.bus.write32(fence + 0x34, 0.05f32.to_bits());
            let mut ticks = 0u32;
            while c.bus.read32(fakes::LOG_COUNT) == 0 {
                fence_update_tick(c, fence);
                ticks += 1;
                assert!(ticks < 100_000, "{hz} Hz: never settles");
            }
            ticks as f32 * 60.0 / hz as f32
        }
        let base = frames(60);
        assert!(base > 20.0, "{base}");
        for hz in [120, 240] {
            let f = frames(hz);
            assert!(
                (f - base).abs() <= 6.0,
                "{hz} Hz: {f} frames vs {base} at 60"
            );
        }
    }

    /// The kick timers `Fence_KickFromNeighbour` and `Fence_v40` arm are 4
    /// frames at 60 Hz: 4 ticks at 60, 8 at 120, 16 at 240.
    #[test]
    fn fence_kick_timers_follow_the_rate() {
        for hz in [60u32, 120, 240] {
            for on in 0..2u64 {
                let mut c = crate::difftest::fresh();
                let c = &mut *c;
                let _ = base(c, 1, 0x34cd_e8);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                crate::tick::set_for_test(c, hz);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = on;
                c.g[RA][0] = sx(crate::difftest::RA);
                fence_kick_from_neighbour(c);
                assert_eq!(c.pc, crate::difftest::RA);
                let timer = c.bus.read32(s + if on == 0 { 0x3c } else { 0x38 });
                assert_eq!(timer, 4 * hz / 60, "kick, {hz} Hz");
            }

            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = base_3(c, 1, 0x0034_d650);
            let fence = DATA + 0x100;
            let impulse = DATA + 0x2000;
            clear_2(c, fence, 0x100);
            for k in 0..16 {
                w_3(c, impulse + 4 * k, r.f32(1.0));
            }
            w_3(c, fence + 0x30, 0.25f32.to_bits());
            let cst = gp(c, -0x2ab8);
            c.bus.write32(cst, 0.5f32.to_bits());
            crate::tick::set_for_test(c, hz);
            c.g[A0][0] = sx(fence);
            c.g[A1][0] = sx(impulse);
            c.g[RA][0] = sx(crate::difftest::RA);
            fence_v40(c);
            assert_eq!(c.pc, crate::difftest::RA);
            assert_eq!(
                c.bus.read32(fence + 0x38),
                4 * hz / 60,
                "v40 +0x38, {hz} Hz"
            );
            assert_eq!(
                c.bus.read32(fence + 0x3c),
                4 * hz / 60,
                "v40 +0x3c, {hz} Hz"
            );
        }
    }

    /// A flag group of 1-6 columns by 1-4 rows in every style (static,
    /// animated, mirrored, pinned/folded, edge-only) with amplitudes of
    /// both signs (the ripples are skipped below the `.lit4` threshold),
    /// real `.lit4` constants and a sine table at 0x504fb8; the flag
    /// manager is reached through the world at gp-0x848.
    #[test]
    fn group_compute_verts_matches() {
        use crate::difftest::{scramble, Rng, GAME_GP};
        check(
            ee::s11::f_0034bca0__generated,
            group_compute_verts,
            300,
            &|c, seed| {
                let mut r = Rng(0x34bca0 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                // World -> +0x84 -> +0x70 = flag manager (wind at +0x10).
                let (world, sub, mgr) = (DATA + 0x3000, DATA + 0x3100, DATA + 0x3200);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, sub);
                c.bus.write32(sub + 0x70, mgr);
                c.bus.write32(mgr + 0x10, r.f32(1.0));
                // The wave table: 0x200 + 0x80 entries of sin(2*pi*i/512).
                for i in 0..0x280u32 {
                    let v = (i as f32 * std::f32::consts::TAU / 512.0).sin();
                    c.bus.write32(0x0050_4fb8 + 4 * i, v.to_bits());
                }
                let g = DATA;
                let style: [(u8, u8); 6] = [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)];
                let (pinned, mirror) = style[seed as usize % 6];
                c.bus.write8(g + GROUP_PINNED, pinned);
                c.bus.write8(g + GROUP_MIRROR, mirror);
                c.bus.write8(g + GROUP_EDGE_ONLY, ((seed / 6) % 2) as u8);
                for off in (0x14..0x4c).step_by(4) {
                    c.bus.write32(g + off, r.f32(3.0));
                }
                c.bus.write32(g + GROUP_WIND_BLEND, r.f32(1.0));
                let cols = 1 + (seed / 12) % 6;
                let rows = if seed % 37 == 0 {
                    0
                } else {
                    1 + (seed / 3) % 4
                };
                let cols = if seed % 41 == 0 { 0 } else { cols };
                c.bus.write32(g + GROUP_COLUMNS, cols);
                c.bus.write32(g + GROUP_ROWS, rows);
                c.bus
                    .write32(g + GROUP_COLUMN_SPAN, ((cols.max(2) - 1) as f32).to_bits());
                c.bus
                    .write32(g + GROUP_ROW_SPAN, ((rows.max(2) - 1) as f32).to_bits());
                let rest = DATA + 0x2000;
                c.bus.write32(g + GROUP_REST_VERTS, rest);
                for i in 0..(6 * 4 * 3) {
                    c.bus.write32(rest + 4 * i, r.f32(50.0));
                }
                for off in (0x78..0x88).step_by(4) {
                    c.bus.write32(g + off, r.f32(6.0));
                }
                c.g[A0][0] = sx(g);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.pc = 0x0034_bca0;
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
        check(
            ee::s11::f_0034c668__generated,
            cflag_manager_update,
            300,
            &|c, seed| {
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
                c.bus
                    .write32(timing + 0x10, [60u32, 30, 1, 2][((seed / 4) % 4) as usize]);
                c.bus.write32(GAME_GP + 0x0a0c, r.next_u32());
                c.bus.write32(
                    this + 0x1c,
                    f([0.5f32, 0.99, 0.0, 0.7][((seed / 16) % 4) as usize]),
                );
                c.bus.write32(
                    this + 0x14,
                    f([0.0f32, 0.5, 1.0, 0.95][((seed / 2) % 4) as usize]),
                );
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
                    c.bus
                        .write32(g + 0x60, if seed % 5 == 0 { 0 } else { mesh });
                    c.bus.write32(g + 0x90, (frame + seed) & 1);
                    c.bus.write32(g + 0x4c, r.edge_f32());
                    c.bus.write32(DATA + 0x4200 + 0x10, r.edge_f32());
                    for i in 0..4 {
                        c.bus.write32(g + 4 + 4 * i, r.edge_f32());
                        c.bus.write32(g + 0x78 + 4 * i, r.edge_f32());
                    }
                    c.bus
                        .write32(g + 0x50, if seed % 4 == 0 { 0 } else { r.f32(1.0) });
                    c.bus
                        .write32(g + 0x54, if seed % 4 == 1 { 0 } else { r.f32(1.0) });
                    c.bus.write32(g + 0x88, r.f32(1.0));
                    c.bus.write32(g + 0x8c, r.f32(1.0));
                    c.bus.write32(GAME_GP.wrapping_sub(0x2b00), 0x3a83_126f); // 0.001
                    let vt = vtable(c, &mut r, &[0x18]);
                    c.bus.write32(mesh + 4, vt);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // (fakes imported above)

    // ------------------------------------------------------ flag manager

    /// cFlagManager_Render: each of the 15 groups with or without models;
    /// the real GroupRender runs and returns at once (no flag manager at
    /// `gp+0x1154` in empty RAM).
    #[test]
    fn flag_manager_render_matches() {
        check(
            ee::s11::f_0034c7f8__generated,
            flag_manager_render,
            200,
            &|c, seed| {
                let mut r = Rng(0x0034_c7f8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0034_c7f8;
                c.g[A0][0] = sx(DATA);
                for i in 0..15u32 {
                    let models = if seed % 7 == 0 || r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(DATA + 0x20 + i * 0x188 + 0x5c, models);
                }
            },
        );
    }

    /// `cFlagManager_GroupSave` with a stream whose two writers are fakes
    /// (some calls unwind), groups of a negative, zero or 1..5 items.
    #[test]
    fn cflag_manager_group_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::fill;
        covered(
            ee::s11::f_0034c2e0__generated,
            cflag_manager_group_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_c2e0, 6);
                c.pc = 0x0034_c2e0;
                let group = DATA;
                let stream = DATA + 0x400;
                let vt = vtable(c, &mut r, &[FLAG_STREAM_WRITE, FLAG_STREAM_WRITE_OBJECT]);
                c.bus.write32(stream, vt);
                let count = match seed % 6 {
                    0 => 0,
                    1 => (-(1 + (r.next_u32() % 4) as i32)) as u32,
                    _ => 1 + r.next_u32() % 5,
                };
                c.bus.write32(group + FLAG_GROUP_COUNT, count);
                fill(c, &mut r, group + FLAG_GROUP_ITEMS, 6);
                c.g[A0][0] = sx(group);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    /// AnimObject_BaseDtor: the object, its cache, buffer and item array
    /// (0..4 items of 0xd0 bytes behind the 0x10-byte `new[]` header) are
    /// allocated from a dlmalloc arena over test memory (pools off) and
    /// freed by the real EA_MemFree; the items' destructors are fakes (some
    /// unwind); the embedded ModObj has no modifier block and no node, so the
    /// real ModObj_dtor takes its short path; flags with and without the
    /// delete bit (the real operator delete frees the object).
    #[test]
    fn anim_object_base_dtor_matches() {
        use crate::difftest::fakes::{FAKE, LOG_COUNT, LOG_UNWIND_AT};
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        check(
            ee::s11::f_0034dba8__generated,
            anim_object_base_dtor,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34dba8, 0);
                // The game heap over test memory, pools off.
                c.bus.write32(GAME_GP + 0xdd8, 0);
                oc238_run(
                    c,
                    0x0031_ff60,
                    ee::s10::f_0031ff60,
                    &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                );
                let alloc = |c: &mut Cpu, size: u64| -> u32 {
                    oc238_run(c, 0x0031_9e48, ee::s10::f_00319e48, &[size, 0]) as u32
                };
                let me = alloc(c, 0x60);
                fill(c, &mut r, me, 0x18);
                // ModObj at +0x14: no node (+4), no instance (+0x18), no block (+0x1c).
                let mod_obj = me + 0x14;
                c.bus.write32(mod_obj + 4, 0);
                c.bus.write16(mod_obj + 0x10, (r.next_u32() % 8) as u16);
                c.bus.write32(mod_obj + 0x18, 0);
                c.bus.write32(mod_obj + 0x1c, 0);
                let cache = if seed % 3 == 0 { 0 } else { alloc(c, 0x20) };
                let buf = if seed % 4 == 1 { 0 } else { alloc(c, 0x30) };
                let items = if seed % 5 == 2 {
                    0
                } else {
                    let n = r.next_u32() % 5;
                    let block = alloc(c, (0x10 + 0xd0 * n) as u64);
                    c.bus.write32(block, n);
                    let items = block + 0x10;
                    let vt = DATA + 0x200;
                    c.bus.write16(vt + 8, ((r.next_u32() % 0x10) & !3) as u16);
                    c.bus.write32(vt + 0xc, FAKE);
                    for i in 0..n {
                        c.bus.write32(items + 0xd0 * i + 0x84, vt);
                    }
                    items
                };
                c.bus.write32(me + 0x44, cache);
                c.bus.write32(me + 0x48, items);
                c.bus.write32(me + 0x4c, buf);
                // Unwind at one of the next calls (the setup made some).
                if seed % 3 == 1 {
                    let n = c.bus.read32(LOG_COUNT);
                    c.bus.write32(LOG_UNWIND_AT, n + r.next_u32() % 8);
                }
                c.g[A0][0] = sx(me);
                c.g[A1][0] = (r.next_u32() & 0xff) as u64;
                c.pc = 0x0034_dba8;
            },
        );
    }

    /// cFlagManager_Save: the real GroupSave on 15 groups of 0..2 items;
    /// the stream's two writers (vtable +0x8 and +0x28) are fakes, some
    /// unwind.
    #[test]
    fn c_flag_manager_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        covered(
            ee::s11::f_0034c848__generated,
            c_flag_manager_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_c848, 30);
                c.pc = 0x0034_c848;
                let this = DATA;
                let stream = DATA + 0x2000;
                let vt = vtable(c, &mut r, &[0x8, 0x28]);
                c.bus.write32(stream, vt);
                for i in 0..15 {
                    let group = this + FLAG_MANAGER_GROUPS + i * FLAG_MANAGER_GROUP_SIZE;
                    let count = match r.next_u32() % 4 {
                        0 => 0u32.wrapping_sub(1),
                        1 => 0,
                        _ => 1 + r.next_u32() % 2,
                    };
                    c.bus.write32(group + FLAG_MANAGER_GROUP_COUNT, count);
                    fill(c, &mut r, group + 0x94, 2);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = stream as u64;
            },
        );
    }

    /// A flag manager at `mgr` with its 15 groups: most unused, some with
    /// 1..4 members on the flag's track or another one; `flag` (its id at
    /// +0x78, its track cell at +0x80) is a member of some. Groups have no
    /// mesh or buffer, so a group emptied frees nothing.
    pub(crate) fn oc308_flag_manager(
        c: &mut Cpu,
        r: &mut crate::difftest::Rng,
        mgr: u32,
        flag: u32,
    ) {
        let track_cell = DATA + 0x2_0000;
        let members = DATA + 0x2_1000;
        let track = r.next_u32();
        c.bus.write32(flag + 0x80, track_cell);
        c.bus.write32(track_cell, track);
        c.bus.write32(flag + 0x78, r.next_u32() % 4);
        for i in 0..FLAG_MANAGER_GROUP_COUNT {
            let g = mgr + FLAG_MANAGER_GROUPS + FLAG_GROUP_SIZE * i;
            let count = if r.next_u32() % 3 == 0 {
                1 + r.next_u32() % 4
            } else {
                0
            };
            c.bus.write32(g + FLAG_GROUP_MEMBERS, count);
            c.bus.write32(
                g + 0x58,
                if r.next_u32() % 3 == 0 {
                    track ^ 1
                } else {
                    track
                },
            );
            c.bus.write32(g + 0x60, 0);
            c.bus.write32(g + 0x74, 0);
            for k in 0..6u32 {
                let m = members + 0x100 * k;
                c.bus.write32(g + 0x94 + 4 * k, m);
                c.bus.write32(m + 0x78, r.next_u32() % 4);
            }
        }
    }

    /// Groups used or not, the flag found in one of them (stops) or none.
    #[test]
    fn c_flag_manager_remove_flag_matches() {
        crate::difftest::check(
            ee::s11::f_0034c600__generated,
            c_flag_manager_remove_flag,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_c600, 0);
                let mgr = DATA;
                let flag = DATA + 0x1_0000;
                oc308_flag_manager(c, &mut r, mgr, flag);
                c.g[A0][0] = sx(mgr);
                c.g[A1][0] = sx(flag);
                c.pc = 0x0034_c600;
            },
        );
    }

    /// Events due inside the group loop (all groups unused: the loop runs
    /// its 15 iterations).
    #[test]
    fn c_flag_manager_remove_flag_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s11::f_0034c600__generated,
            c_flag_manager_remove_flag,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34_c6e0, 0);
                let mgr = DATA;
                let flag = DATA + 0x1_0000;
                oc308_flag_manager(c, &mut r, mgr, flag);
                if seed % 2 == 0 {
                    for i in 0..FLAG_MANAGER_GROUP_COUNT {
                        c.bus.write32(
                            mgr + FLAG_MANAGER_GROUPS + FLAG_GROUP_SIZE * i + FLAG_GROUP_MEMBERS,
                            0,
                        );
                    }
                }
                c.g[A0][0] = sx(mgr);
                c.g[A1][0] = sx(flag);
                schedule(c, &mut r, seed, 0x0034_c600)
            },
        );
    }

    /// AnimObject_BaseCtor: the ModObj_ctor fixture (process lists, world ->
    /// focus manager, node flags with or without bit 0 / 0x100) plus 0..4
    /// node definitions with or without mesh sources (so the mesh duration
    /// branch runs or not), durations from a clip or none, `force` on or
    /// off; every callee (operator_new_named on the test pools / heap,
    /// AllocNodeInfo, GetDuration, v52 -> IntersectInfo_EvaluateMatrix) runs
    /// for real on the ELF image.
    #[test]
    fn anim_object_base_ctor_matches() {
        use crate::difftest::{check, elf_image, fill, DATA, GAME_GP};
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const SELF: u32 = DATA + 0x100;
        const NODE: u32 = DATA + 0x200;
        const DEF: u32 = DATA + 0x400;
        const ENTRIES: u32 = DATA + 0x500;
        const SOURCES: u32 = DATA + 0x600;
        const CURVES: u32 = DATA + 0x800;
        const ACTION: u32 = DATA + 0x840;
        const CLIP: u32 = DATA + 0x880;
        const PROCS: u32 = DATA + 0x1000;
        const TAILS: u32 = DATA + 0x1400;
        const WORLD: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        check(
            ee::s11::f_0034d9b0__generated,
            anim_object_base_ctor,
            120,
            &|c, seed| {
                elf_image(c);
                let mut r = alloc_machine(c, seed, 0x34_d9b0);
                c.pc = 0x0034_d9b0;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                for i in 0..0x40 {
                    w(c, SELF + 4 * i, 0);
                }
                fill(c, &mut r, NODE, 0x20);
                let flags = r.next_u32();
                w(
                    c,
                    NODE + 0x8,
                    if seed % 4 == 0 {
                        flags | 0x100
                    } else {
                        flags & !0x100
                    },
                );
                w(c, NODE + 0x80, DEF);
                let defs = seed % 5;
                w(c, DEF + 4, defs);
                w(c, DEF + 8, ENTRIES);
                for i in 0..defs {
                    let src = if seed % 7 == 1 || r.next_u32() % 3 == 0 {
                        0
                    } else {
                        SOURCES + 0x40 * i
                    };
                    w(c, ENTRIES + 0x10 * i + 8, src);
                    for k in 0..6 {
                        w(c, SOURCES + 0x40 * i + 4 * k, r.f32(100.0));
                    }
                    w(c, SOURCES + 0x40 * i + 0x18, 0); // no animated channel
                    w(c, SOURCES + 0x40 * i + 0x20, CURVES);
                }
                w(c, CURVES, if seed % 3 == 0 { 0 } else { ACTION });
                w(c, ACTION + 4, if seed % 5 == 2 { 0 } else { CLIP });
                w(c, CLIP + 0x10, r.f32(30.0));
                // The ModObj_ctor fixture: process lists, world -> focus.
                let list = seed % 4;
                w(c, GAME_GP + 0x2898 + 4, PROCS);
                for i in 0..4 {
                    w(c, PROCS + 0x44 * i, r.next_u32());
                    w(c, PROCS + 0x44 * i + 8, TAILS + 0x40 * i);
                    w(c, PROCS + 0x44 * i + 0x28, TAILS + 0x40 * i + 0x20);
                }
                let kind = match (seed / 4) % 6 {
                    0 => 6,
                    1 => 0x10,
                    2 => 0x16,
                    _ => r.next_u32() % 0x20,
                };
                fill(c, &mut r, 0x0044_aff0, 0x20);
                w(c, GAME_GP - 0x848, WORLD);
                w(c, WORLD + 0x84, WORLD + 0x100);
                w(c, WORLD + 0x100 + 0xc, WORLD + 0x200);
                w(c, WORLD + 0x200 + 0xa4, FOCUS);
                w(
                    c,
                    FOCUS + 0xd4,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                w(c, FOCUS + 0xd8, r.next_u32() % 8);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(kind);
                c.g[A3][0] = sx(NODE);
                c.g[T0][0] = if seed % 3 == 1 {
                    sx(r.next_u32() | 1)
                } else {
                    0
                };
            },
        );
    }
}
