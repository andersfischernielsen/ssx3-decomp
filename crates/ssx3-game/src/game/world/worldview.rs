//! `src/world/worldview.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::bx::bxstringctor::vdot4;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::object::splinemodifier::lwc1_r;
pub(crate) use crate::game::object::splinemodifier::swc1_r;
pub(crate) use crate::game::render::particle::add32;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::render::particle::fconst;
pub(crate) use crate::game::render::particle::fr;
pub(crate) use crate::game::render::particle::gp;
pub(crate) use crate::game::render::particle::ldf;
pub(crate) use crate::game::render::particle::le;
pub(crate) use crate::game::render::particle::lt;
pub(crate) use crate::game::render::particle::p;
pub(crate) use crate::game::render::particle::setf;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003a_9820,
        c_world_cache_streamer_get_section,
        "cWorldCache_StreamerGetSection",
    ),
    (
        0x003a_9858,
        c_world_cache_streamer_is_section_resident,
        "cWorldCache_StreamerIsSectionResident",
    ),
    (
        0x003a_9890,
        c_world_cache_streamer_is_section_unloaded,
        "cWorldCache_StreamerIsSectionUnloaded",
    ),
    (
        0x003a_99d8,
        c_world_cache_streamer_activate_section,
        "cWorldCache_StreamerActivateSection",
    ),
    (
        0x003a_9a98,
        c_world_cache_streamer_get_hull,
        "cWorldCache_StreamerGetHull",
    ),
    (
        0x003a_9ab0,
        c_world_cache_streamer_get_hull_count,
        "cWorldCache_StreamerGetHullCount",
    ),
    (
        0x003a_a028,
        world_cache_streamer_update_view_node,
        "cWorldCache_StreamerUpdateViewNode",
    ),
    (
        0x003a_a3f0,
        c_ps2_chunk_resolve_base_v05,
        "cPS2ChunkResolveBase_v05",
    ),
    (0x003a_a438, c_ps2_chunk_resolve_v06, "cPS2ChunkResolve_v06"),
    (0x003a_a520, cps2_chunk_resolve_v07, "cPS2ChunkResolve_v07"),
    (0x003a_a5c8, ps2_chunk_resolve_v14, "cPS2ChunkResolve_v14"),
    (0x003a_a608, c_ps2_chunk_resolve_v15, "cPS2ChunkResolve_v15"),
    (0x003a_a6e8, cps2_chunk_resolve_v16, "cPS2ChunkResolve_v16"),
    (0x003a_a700, cps2_chunk_resolve_v17, "cPS2ChunkResolve_v17"),
    (0x003a_a758, cps2_chunk_resolve_v09, "cPS2ChunkResolve_v09"),
    (0x003a_a780, cps2_chunk_resolve_v10, "cPS2ChunkResolve_v10"),
    (0x003a_a830, cps2_chunk_resolve_v11, "cPS2ChunkResolve_v11"),
    (0x003a_a8d8, cps2_chunk_resolve_v12, "cPS2ChunkResolve_v12"),
    (
        0x003a_a8f0,
        cps2_chunk_resolve_base_v13,
        "cPS2ChunkResolveBase_v13",
    ),
    (0x003a_a960, cps2_chunk_resolve_v18, "cPS2ChunkResolve_v18"),
    (0x003a_abd8, cps2_chunk_resolve_v20, "cPS2ChunkResolve_v20"),
    (0x003a_ac50, cps2_chunk_resolve_v21, "cPS2ChunkResolve_v21"),
    (0x003a_aca8, cps2_chunk_resolve_v22, "cPS2ChunkResolve_v22"),
    (
        0x003a_ad98,
        c_ps2_chunk_resolve_create,
        "cPS2ChunkResolve_Create",
    ),
    (
        0x003a_ade8,
        c_ps2_chunk_resolve_dtor,
        "cPS2ChunkResolve_dtor",
    ),
    (
        0x003a_ae40,
        cps2_chunk_resolve_init,
        "cPS2ChunkResolve_Init",
    ),
    (0x003a_be40, cps2chunkresolve_v05, "cPS2ChunkResolve_v05"),
    (0x003a_c2a8, cps2_chunk_resolve_v13, "cPS2ChunkResolve_v13"),
    (
        0x003a_c358,
        cps2chunkresolve_relocate_instance_dma,
        "cPS2ChunkResolve_RelocateInstanceDma",
    ),
    (0x003a_c7c8, cw_script_cache_ctor, "cWScriptCache_ctor"),
    (
        0x003a_9e50,
        world_cache_point_box_distance,
        "cWorldCache_PointBoxDistance",
    ),
    (
        0x003a_ae68,
        c_ps2_chunk_resolve_resolve,
        "cPS2ChunkResolve_Resolve",
    ),
    (
        0x003a_b498,
        cps2_chunk_resolve_release,
        "cPS2ChunkResolve_Release",
    ),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {

    pub(crate) use crate::game::bx::bxstringctor::callee::CHUNK_RESOLVE_RELOCATE_DMA;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::CW_SCRIPT_CACHE_ALLOC;

    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;

    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;

    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;

    pub(crate) use crate::game::render::particle::callee::IS_HULL_RESIDENT;

    pub(crate) use crate::game::render::particle::callee::POINT_BOX_DISTANCE;

    pub(crate) use crate::game::render::particle::callee::REQUEST_HULL;

    pub(crate) use crate::game::render::particle::callee::STREAMER_UPDATE_VIEW_NODE;

    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;

    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
}

// ---------------------------------------------------------------------------
// World streamer.

/// Streamer view records: 0x50 bytes each at `wc + 0x250`.
pub(crate) mod view {
    pub(crate) use crate::game::main::game::view::BOX;
    pub(crate) use crate::game::main::game::view::EYE;
    pub(crate) use crate::game::main::game::view::HULL_DIST;
    pub(crate) use crate::game::main::game::view::HULL_RANGE;
    pub(crate) use crate::game::main::game::view::LEFT_POINT;
    pub(crate) use crate::game::main::game::view::NEAREST;
    pub(crate) use crate::game::main::game::view::RIGHT_POINT;
    pub(crate) use crate::game::main::game::view::STRIDE;
}

/// Streamer tree node fields.
pub(crate) mod node {

    pub const BOX_MIN: u32 = 0x00;

    pub const BOX_MAX: u32 = 0x10;

    pub const LEFT_PLANE: u32 = 0x20;

    pub const RIGHT_PLANE: u32 = 0x30;

    pub const LEFT: u32 = 0x50;

    pub const RIGHT: u32 = 0x54;

    pub const HULL: u32 = 0x58;
}

// ---------------------------------------------------------------------------
// World cache streamer sections.

/// `Section* cWorldCache_StreamerGetSection(Streamer* self, int index)`
/// (0x3a9820): look the section up in the table at self+8; records are 0x58
/// bytes.
pub fn c_world_cache_streamer_get_section(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = self_.off(0x4).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x58;
        if in_range {
            c.cycles += 4;
            c.g[V1][0] = self_.off(0x8).lw(c);
            c.mult(0, V0, A1, V0);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `bool cWorldCache_StreamerIsSectionResident(Streamer* self, int index)`
/// (0x3a9858): true when the section's state at +0x14 is 3 or 4.
pub fn c_world_cache_streamer_is_section_resident(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = self_.off(0x4).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
        if in_range {
            c.cycles += 5;
            c.g[V1][0] = sx(self_.0.wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(5));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `bool cWorldCache_StreamerIsSectionUnloaded(Streamer* self, int index)`
/// (0x3a9890): true when the section's state at +0x14 is zero.
pub fn c_world_cache_streamer_is_section_unloaded(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V0][0] = self_.off(0x4).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
        if in_range {
            c.cycles += 4;
            c.g[V1][0] = sx(self_.0.wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
            c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `Hull* cWorldCache_StreamerGetHull(Streamer* self, int index)`
/// (0x3a9a98): base of the hull array at self+0x10 plus index*0x44.
pub fn c_world_cache_streamer_get_hull(c: &mut Cpu) {
    c.cycles += 5;
    let self_ = abi::arg(c, 0);
    c.g[V1][0] = 0x44;
    c.g[V0][0] = Addr(self_.wrapping_add(0x10)).lw(c);
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `int cWorldCache_StreamerGetHullCount(Streamer* self)` (0x3a9ab0): hull
/// count at the streamer's owner + 0x10.
pub fn c_world_cache_streamer_get_hull_count(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = abi::arg(c, 0);
    c.g[V1][0] = Addr(self_.wrapping_add(0x4)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
    abi::ret(c);
}

/// `v1 = view * 0x50` (`mult`, LO/HI as the original), `v0 = wc + v1`:
/// the view record (v0 = 0x50 on entry).
#[inline(always)]
pub(crate) fn view_record(c: &mut Cpu) -> u32 {
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
pub(crate) fn streamer_visit(c: &mut Cpu, frame: Frame) -> Result<bool, crate::call::Unwound> {
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
        (
            node::LEFT,
            node::LEFT_PLANE,
            view::LEFT_POINT,
            0x003a_a250u32,
        ),
        (
            node::RIGHT,
            node::RIGHT_PLANE,
            view::RIGHT_POINT,
            0x003a_a2a8,
        ),
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
pub(crate) fn hull_id(c: &mut Cpu, nd: u32) {
    c.g[V0][0] = c.lw(p(nd, node::HULL));
    c.g[A0][0] = c.lw(r32(c, S2));
    c.g[A1][0] = c.lhu(at(c, V0, 2));
}

/// `void* cWScriptCache_ctor(cWScriptCache* self, int count)` (0x3ac7c8):
/// stash the world, zero the list and allocate `count` script entries.
pub fn cw_script_cache_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let world = c.g[A1][0];
        Addr(this).sw(c, world);
        Addr(this).off(4).sw(c, 0);
        c.g[A1][0] = 1;
        callee::CW_SCRIPT_CACHE_ALLOC.call(c, 0x003a_c7e8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2ChunkResolve_v14(void* self, void* chunk, int a2)` (0x3AA5C8): make
/// the chunk's relative pointer at +0x8 absolute and let the owner's virtual
/// handler at +0x7C resolve it; returns 1.
pub fn ps2_chunk_resolve_v14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        let this = r(c, A0);
        let chunk = c.g[A1][0];
        c.g[V1][0] = chunk;
        frame.save_ra(c, 0);
        let offset = c.lw_r_2(V0, (chunk as u32).wrapping_add(8));
        let absolute = sx((offset as u32).wrapping_add(chunk as u32));
        c.g[V0][0] = absolute;
        Addr((chunk as u32).wrapping_add(8)).sw(c, absolute);
        let owner = c.lw_r_2(A2, this.wrapping_add(8));
        let delta = c.lh_r(V1, (owner as u32).wrapping_add(0x78));
        let target = c.lw_r_2(V0, (owner as u32).wrapping_add(0x7c));
        let t = target as u32;
        let a0 = sx(this.wrapping_add(delta as u32));
        c.g[A0][0] = a0;
        call_ptr(c, t, 0x003a_a5f4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2ChunkResolveBase_v05(self, chunk)` (0x3aa3f0): relocate the
/// chunk's two internal pointers to absolute addresses, hand the chunk to the
/// resolver's slot 0x30/0x34 and report success.
pub fn c_ps2_chunk_resolve_base_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let chunk = c.g[V0][0] as u32;
        c.g[A2][0] = Addr(chunk).off(8).lw(c);
        c.g[V1][0] = Addr(chunk).off(0xc).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(chunk));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(chunk));
        Addr(chunk).off(8).sw(c, c.g[A2][0]);
        Addr(chunk).off(0xc).sw(c, c.g[V1][0]);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        let resolver = c.g[A2][0] as u32;
        c.g[V1][0] = Addr(resolver).off(0x30).lh(c);
        c.g[V0][0] = Addr(resolver).off(0x34).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x003a_a428);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x003a_a428)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `void* cPS2ChunkResolve_Create(void)` (0x3aad98): allocate and construct a
/// chunk resolver.
pub fn c_ps2_chunk_resolve_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x49_4f70; // D_00494F70
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x003a_adbc);
        c.g[A0][0] = 0x14;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x003a_adbc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::CPS2_CHUNK_RESOLVE_CTOR.call(c, 0x003a_adc8)?;
        c.g[V1][0] = 0x49_5090; // D_00495090
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        Addr(r32(c, S0)).off(8).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2ChunkResolve_dtor(cPS2ChunkResolve* self, int flags)` (0x3aade8):
/// two-stage teardown with an optional free.
pub fn c_ps2_chunk_resolve_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx(0x0049_5090);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x003a_ae10);
        Addr(r32(c, S1)).off(0x8).sw(c, c.g[V0][0]);
        callee::CPS2_CHUNK_RESOLVE_DTOR_STUB.call(c, 0x003a_ae10)?;
        c.g[V0][0] = 0x0049_0000;
        c.g[S0][0] &= 0x1;
        c.g[V0][0] = sx(0x0049_5150);
        let keep = c.g[S0][0] == 0;
        Addr(r32(c, S1)).off(0x8).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[RA][0] = sx(0x003a_ae2c);
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE_2.call(c, 0x003a_ae2c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `base + off` (a 32-bit address add).
#[inline(always)]
pub(crate) fn p_2(base: u32, off: u32) -> u32 {
    base.wrapping_add(off)
}

/// `int cPS2ChunkResolve_v16(void* self, ResolveNode* node)` (0x003aa6e8):
/// the node's `+0x20` word becomes its own address (a self-relative base).
pub fn cps2_chunk_resolve_v16(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    c.g[V1][0] = Addr(p_2(node, 0x20)).lw(c);
    c.g[V0][0] = 1;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(node));
    c.sw(p_2(node, 0x20), c.g[V1][0]);
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v12(void* self, ResolveNode* node, int v)`
/// (0x003aa8d8): `node->base += v`.
pub fn cps2_chunk_resolve_v12(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    let v = abi::arg(c, 2);
    c.g[V1][0] = Addr(p_2(node, 0x4)).lw(c);
    c.g[V0][0] = 1;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(v));
    c.sw(p_2(node, 0x4), c.g[V1][0]);
    abi::ret(c);
}

/// `int cPS2ChunkResolve_Init(cPS2ChunkResolve* self, int n)` (0x003aae40):
/// remember the world and the chunk count.
pub fn cps2_chunk_resolve_init(c: &mut Cpu) {
    c.cycles += 8;
    let self_ = abi::arg(c, 0);
    let n = abi::arg(c, 1);
    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8)); // the world
    c.sw(p_2(self_, 0xc), n as u64);
    c.sw(self_, c.g[A2][0]);
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2a));
    c.sw(p_2(self_, 0x10), c.g[A1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve: the per-kind hash lookups.

/// `int cPS2ChunkResolve_v17(void* self, ResolveNode* node)` (0x003aa700):
/// resolve the hash at `node+0x64` through the `+0x24` table, store the
/// result and clear `node+0x84`.
pub fn cps2_chunk_resolve_v17(c: &mut Cpu) {
    c.cycles += 10;
    let self_ = abi::arg(c, 0);
    let node = abi::arg(c, 1);
    c.g[A2][0] = Addr(self_).lw(c);
    c.g[A3][0] = Addr(p_2(node, 0x64)).lw(c);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c); // *(self)
    c.g[V0][0] = c.g[A3][0] & 0xff;
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8)); // section table
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32); // section
    let h = c.g[A3][0];
    if c.g[V1][0] != 0 {
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        c.cycles += 7;
        c.g[V0][0] = sx((h as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let hit = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !hit {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    } else {
        c.cycles += 2;
        c.g[V1][0] = 0;
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 4;
    c.sw(p_2(node, 0x64), c.g[V0][0]);
    c.g[V0][0] = 1;
    c.sw(p_2(node, 0x84), 0);
    abi::ret(c);
}

/// `int cPS2ChunkResolveBase_v13(void* self, ResolveNode* node)`
/// (0x003aa8f0): resolve `node+0x80` through the `+0x14` table, then seed
/// `node+0x8c`/`+0x94` from the resolved structure.
pub fn cps2_chunk_resolve_base_v13(c: &mut Cpu) {
    c.cycles += 10;
    let self_ = abi::arg(c, 0);
    let node = abi::arg(c, 1);
    c.g[A2][0] = Addr(self_).lw(c);
    c.g[A3][0] = Addr(p_2(node, 0x80)).lw(c);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c); // *(self)
    c.g[V0][0] = c.g[A3][0] & 0xff;
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8)); // section table
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32); // section
    let h = c.g[A3][0];
    if c.g[V1][0] == 0 {
        // No section: the node keeps the null pointer (v1 = 0).
        c.cycles += 2;
        c.g[V1][0] = 0;
    } else {
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        c.cycles += 7;
        c.g[V0][0] = sx((h as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        }
        c.cycles += 1;
        c.g[V1][0] = c.g[V0][0];
    }
    c.cycles += 6;
    c.sw(p_2(node, 0x80), c.g[V1][0]);
    c.g[V0][0] = 1;
    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.sw(p_2(node, 0x8c), 0);
    c.sw(p_2(node, 0x94), c.g[V1][0]);
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v09(void* self, ResolveNode* node)` (0x003aa758):
/// point `node+0x10` at `node+0x14` (or 0 when it was -1).
pub fn cps2_chunk_resolve_v09(c: &mut Cpu) {
    c.cycles += 4;
    let node = abi::arg(c, 1);
    c.g[V0][0] = Addr(p_2(node, 0x10)).lw(c);
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    let unset = c.g[V0][0] == c.g[V1][0];
    c.g[V0][0] = sx(p_2(node, 0x14));
    if unset {
        c.cycles += 2;
        c.sw(p_2(node, 0x10), 0);
    } else {
        c.cycles += 1;
        c.sw(p_2(node, 0x10), c.g[V0][0]);
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v10(void* self, int n, u32* data)` (0x003aa780):
/// resolve every hash of `n` counted arrays in place; `data` advances by
/// the array's own length.
pub fn cps2_chunk_resolve_v10(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T3][0] = c.g[A0][0]; // self
    let n = c.g[A1][0];
    c.g[A0][0] = 0; // g
    if (n as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32); // *data
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
        let count = c.g[V0][0];
        c.g[T1][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 1;
            loop {
                // h = *hp; look it up; *hp = res
                c.cycles += 10;
                c.g[V1][0] = c.lw(c.g[T3][0] as u32); // *(self)
                c.g[A3][0] = c.lw(c.g[T0][0] as u32); // h
                c.g[A0][0] = c.lw(c.g[V1][0] as u32); // *(*(self))
                c.g[V0][0] = c.g[A3][0] & 0xff;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32); // section
                let h = c.g[A3][0];
                let missing = c.g[V1][0] == 0;
                c.g[V0][0] = sx((h as u32) >> 8);
                let mut res = 0u64;
                if missing {
                    c.cycles += 1;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        res = c.g[V0][0];
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = res;
                c.sw(c.g[T0][0] as u32, c.g[V0][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_a7a8);
                }
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[T2][0];
        c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        let more = c.g[V1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_a790);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v18(void* self, List* list)` (0x003aa960): rebuild
/// the ring indices (`prev`/`next` at +0x60/+0x64, owner at +0x68) of every
/// node of the list at +0x30.
pub fn cps2_chunk_resolve_v18(c: &mut Cpu) {
    c.cycles += 7;
    let list = abi::arg(c, 1);
    c.g[V0][0] = Addr(p_2(list, 0x20)).lw(c);
    c.g[T5][0] = sx(p_2(list, 0x30)); // nodes
    c.g[T1][0] = 1; // next index
    c.sw(p_2(list, 0x24), c.g[T5][0]);
    c.g[T0][0] = 0; // i
    let count = c.g[V0][0];
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1)); // prev index
    if (count as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[T3][0] = 0x90; // node stride
    c.g[T2][0] = 0xffff_ffff_ffff_ffff;
    c.g[A2][0] = c.g[T5][0];
    loop {
        // node->prev: -1 becomes 0, else &nodes[prev]
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x60));
        let unset = c.g[V0][0] == c.g[T2][0];
        c.mult(0, 3, 7, 11); // prev * 0x90 -> v1
        if unset {
            c.cycles += 1;
            c.sw((c.g[A2][0] as u32).wrapping_add(0x60), 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T5][0] as u32));
            c.sw((c.g[A2][0] as u32).wrapping_add(0x60), c.g[V0][0]);
        }
        // node->next: the same for +0x64
        c.cycles += 6;
        c.g[A0][0] = 0x90;
        c.mult(0, 2, 8, 4); // i * 0x90 -> v0
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
        c.mult(0, 12, 9, 4); // next * 0x90 -> t4
        if c.g[V0][0] == c.g[T2][0] {
            c.cycles += 1;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[T5][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
        }
        c.cycles += 13;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x68), c.g[A1][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[A3][0] = 0;
        }
        if c.g[A0][0] == 0 {
            c.g[T1][0] = 0;
        }
        c.g[V1][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x90));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_a988);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v21(void* self, ResolveNode* node)` (0x003aac50):
/// clamp the resolved object's value at +0x1c to 5000 while it is still
/// initialising (0 < state < 3).
pub fn cps2_chunk_resolve_v21(c: &mut Cpu) {
    let node = abi::arg(c, 1);
    c.cycles += 4;
    c.g[V0][0] = Addr(p_2(node, 0xc)).lw(c);
    c.g[V0][0] = c.g[V0][0] & 0x100;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[V1][0] = Addr(p_2(node, 0x10)).lw(c);
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if (c.g[V1][0] as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.lwc1(0, p_2(node, 0x1c));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x23b8)); // 5000.0f
    c.c_lt_s(1, 0); // 5000 < value
    if c.fcc() {
        c.swc1(1, p_2(node, 0x1c));
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v22(void* self, ResolveNode* node)` (0x003aaca8):
/// true while the object is initialising (state 1 or 2) and not flagged.
pub fn cps2_chunk_resolve_v22(c: &mut Cpu) {
    let node = abi::arg(c, 1);
    c.cycles += 4;
    c.g[V0][0] = Addr(p_2(node, 0xc)).lw(c);
    c.g[V0][0] = c.g[V0][0] & 0x100;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[A1][0] = Addr(p_2(node, 0x10)).lw(c);
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A1][0] as i64) < 3) as u64;
    let below3 = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if below3 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        if (c.g[A1][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cWorldCache_StreamerActivateSection(cWorldCache* cache, int i)`
/// (0x3a99d8): move section `i`'s streamer state from 5 (requested) to 6
/// (active), flagging its objects; returns 1 when active.
pub fn c_world_cache_streamer_activate_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_3(r32(c, A3), 4)).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(p_3(c.g[V0][0] as u32, 8)).lw(c);
            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
            if !out_of_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = 5;
                c.g[A2][0] = Addr(c.g[V1][0] as u32).lw(c);
                let requested = c.g[A2][0] == c.g[A0][0];
                if !requested {
                    c.g[V1][0] = 6;
                    c.cycles += 2;
                    let active = c.g[A2][0] == c.g[V1][0];
                    c.g[V0][0] = 1;
                    if active {
                        // returns 1
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 6;
                    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
                    c.g[A0][0] = Addr(c.g[A3][0] as u32).lw(c);
                    call::call_ptr(c, 0x003a_8f10, 0x003a_9a28)?;
                    c.g[V0][0] = 1;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `func_003AABD8` (0x3aabd8): `cPS2ChunkResolve`. Runs the chunk resolve hook
/// on the chunk's flags (`*(this+0xc) & 0x70`), then fills the bounding box at
/// `+0x28`..`+0x3c` and `+0x34`..`+0x44` with the centre (`+0x1c`..`+0x24`) plus
/// and minus the half-extent already in `f0`; `v0` is left 1.
pub fn cps2_chunk_resolve_v20(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A0][0] = lw_r(c, S0, 0xc);
        c.g[RA][0] = sx(0x003a_abf4);
        c.g[A0][0] = c.g[A0][0] & 0x70;
        callee::WORLD_CHUNK_RESOLVE_HOOK.call(c, 0x003a_abf4)?;
        lwc1_r(c, 3, S0, 0x1c);
        c.g[V0][0] = 0x1;
        lwc1_r(c, 2, S0, 0x20);
        lwc1_r(c, 1, S0, 0x24);
        c.f[4] = (FpuF32(c.f[3]) + FpuF32(c.f[0])).0;
        c.f[5] = (FpuF32(c.f[2]) + FpuF32(c.f[0])).0;
        sw_r(c, S0, 0x44, 0);
        c.f[6] = (FpuF32(c.f[1]) + FpuF32(c.f[0])).0;
        sw_r(c, S0, 0x40, 0);
        c.f[3] = (FpuF32(c.f[3]) - FpuF32(c.f[0])).0;
        swc1_r(c, 4, S0, 0x34);
        c.f[2] = (FpuF32(c.f[2]) - FpuF32(c.f[0])).0;
        swc1_r(c, 5, S0, 0x38);
        c.f[1] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
        swc1_r(c, 6, S0, 0x3c);
        swc1_r(c, 3, S0, 0x28);
        swc1_r(c, 2, S0, 0x2c);
        swc1_r(c, 1, S0, 0x30);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the two chunk relocators: add `base` to every pointer
/// in the chunk and fix each entry through the object's method at `slot`.
pub(crate) fn chunk_resolve(c: &mut Cpu, slot: u32, entry: u32) -> Flow {
    c.cycles += 17;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    frame.save(c, S2, 0x30);
    c.g[S1][0] = 0;
    frame.save(c, S3, 0x20);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S4, 0x10);
    c.g[S3][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S4][0] = c.g[A0][0];
    c.g[V0][0] = Addr(p(r32(c, S3), 0x20)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, S3), 0x1c)).lw(c);
    c.g[V0][0] = sx(p(r32(c, V0), r32(c, S2)));
    c.g[S0][0] = c.g[V0][0];
    let none = (c.g[V1][0] as i64) <= 0;
    Addr(p(r32(c, S3), 0x20)).sw(c, c.g[V0][0]);
    if !none {
        c.cycles += 1;
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        loop {
            c.cycles += 14;
            c.g[A2][0] = c.g[S2][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.g[A1][0] = sx(p(r32(c, A1), r32(c, S2)));
            Addr(r32(c, S0)).sw(c, c.g[A1][0]);
            c.g[S0][0] = sx(p(r32(c, S0), 4));
            c.g[V1][0] = Addr(p(r32(c, S4), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), slot)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), slot + 4)).lw(c);
            let fix = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S4), r32(c, A0)));
            call_ptr(c, fix, entry + 0x70)?;
            c.g[V1][0] = Addr(p(r32(c, S3), 0x1c)).lw(c);
            c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V1][0] == 0 {
                break;
            }
            c.g[A1][0] = Addr(r32(c, S0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, entry + 0x48);
            }
        }
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x50);
    c.g[V0][0] = 1;
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int cPS2ChunkResolve_v07(self, chunk, base)` (0x3aa520).
pub fn cps2_chunk_resolve_v07(c: &mut Cpu) {
    guest(c, |c| chunk_resolve(c, 0x40, 0x003a_a520))
}

/// `int cPS2ChunkResolve_v11(self, chunk, base)` (0x3aa830).
pub fn cps2_chunk_resolve_v11(c: &mut Cpu) {
    guest(c, |c| chunk_resolve(c, 0x60, 0x003a_a830))
}

// ---------------------------------------------------------------------------
// `cPS2ChunkResolve_v13` (0x3ac2a8): resolve a chunk instance through the
// two-level lookup tables, write the resolved pointers/flags into the
// instance and relocate its DMA chain. Returns 0 when the id has no
// resource, 1 when it does.
pub fn cps2_chunk_resolve_v13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A1][0];
        c.g[V1][0] = c.lw(c.g[T0][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x80));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        let empty = c.g[V1][0] == 0;
        c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
        // The generated body has no early return: both misses charge one cycle,
        // zero V0 and fall into the relocate block below.
        if !empty {
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let miss = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if miss {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 21;
        c.sw((c.g[A3][0] as u32).wrapping_add(0x80), c.g[V0][0]);
        c.g[A2][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] = c.g[T0][0];
        c.g[A1][0] = c.g[A3][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.sw((c.g[A3][0] as u32).wrapping_add(0x8c), 0u64);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x94), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[A3][0] as u32).wrapping_add(0x88), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x2;
        c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        callee::CHUNK_RESOLVE_RELOCATE_DMA.call(c, 0x003a_c344)?;
        c.g[V0][0] = 0x1;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Chunk resolver.

/// `int cPS2ChunkResolve_v05(self, void* chunk)` (0x3abe40): relocate the
/// chunk's two offsets and every pointer table entry, then finish the mesh
/// DMA relocation.
pub fn cps2chunkresolve_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
        c.g[V0][0] = sx(p(r32(c, V0), r32(c, S0)));
        c.g[T0][0] = sx(p(r32(c, V1), r32(c, S0)));
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[T0][0]);
        c.g[V0][0] = Addr(r32(c, T0)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = sx(p(r32(c, T0), 4));
        if !none {
            loop {
                c.cycles += 10;
                c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                c.g[A1][0] = Addr(r32(c, A2)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = c.g[A1][0] & 0xff;
                c.g[V1][0] = Addr(p(r32(c, A0), 8)).lw(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(p(r32(c, V0), r32(c, V1)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                let empty = c.g[V1][0] == 0;
                c.g[V0][0] = sx(r32(c, A1) >> 8);
                let mut resolved = 0u64;
                if !empty {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(p(r32(c, V1), 4)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(p(r32(c, V0), r32(c, A0)));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    if c.g[V0][0] != 0 {
                        resolved = sx(r32(c, V0) << 2);
                    }
                }
                c.cycles += 6;
                Addr(r32(c, A2)).sw(c, resolved);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.g[V0][0] = Addr(r32(c, T0)).lw(c);
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(p(r32(c, A2), 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_be80);
                }
            }
        }
        c.cycles += 15;
        c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x34)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x003a_bef8)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x003a_bf20, 0x003a_bf04)?;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2ChunkResolve_RelocateInstanceDma(self, void* chunk)` (0x3ac358):
/// relocate the instance-DMA chain for every instance in the chunk.
pub fn cps2chunkresolve_relocate_instance_dma(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S7, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[A0][0] = 0;
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S2), 0x98)).lw(c);
        c.g[S5][0] = Addr(p(r32(c, S2), 0x80)).lw(c);
        c.g[V0][0] = sx(p(r32(c, V0), r32(c, S2)));
        Addr(p(r32(c, S2), 0x98)).sw(c, c.g[V0][0]);
        c.g[A3][0] = c.g[V0][0];
        c.g[S6][0] = Addr(p(r32(c, S5), 4)).lw(c);
        let none = (c.g[S6][0] as i64) <= 0;
        c.g[V1][0] = Addr(p(r32(c, S5), 8)).lw(c);
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[S1][0] = Addr(p(r32(c, V1), 4)).lw(c);
                c.g[S4][0] = sx(r32(c, A0).wrapping_add(1));
                let no_obj = c.g[S1][0] == 0;
                c.g[S3][0] = sx(p(r32(c, V1), 0x10));
                let count_le;
                if !no_obj {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
                    c.g[S0][0] = 0;
                    count_le = (c.g[V0][0] as i64) <= 0;
                } else {
                    count_le = true;
                }
                if !count_le {
                    c.cycles += 1;
                    loop {
                        c.cycles += 9;
                        c.g[A0][0] = c.g[S7][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S5][0];
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        call_ptr(c, 0x003a_c508, 0x003a_c3e4)?;
                        c.g[V1][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
                        c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        let more = c.g[V1][0] != 0;
                        c.g[A3][0] = c.g[V0][0];
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_c3d0);
                        }
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S4][0];
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = c.g[S3][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_c3b0);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x80);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_v15 (0x00003aa608).

/// `cPS2ChunkResolve_v15`: resolve a PlayStation 2 chunk descriptor (version 15) to data pointers.
pub fn c_ps2_chunk_resolve_v15(c: &mut Cpu) {
    guest(c, |c| {
        'b_3aa6c0: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S2][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            let t = (c.g[V1][0] as i64) <= 0;
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            if t {
                break 'b_3aa6c0;
            } else {
                c.cycles += 5;
                c.g[S4][0] = 0xffffffffffff0000;
                c.g[V0][0] = 0x500000;
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
                c.g[S3][0] = c.g[V1][0];
                c.g[S4][0] = c.g[S4][0] | 0xffff;
                'l_3aa650: loop {
                    'b_3aa674: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        let t = c.g[A1][0] == 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        if t {
                            break 'b_3aa674;
                        } else {
                            c.cycles += 6;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[RA][0] = sx(0x003aa674);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x003aa674 {
                                return Err(crate::call::Unwound);
                            }
                            break 'b_3aa674;
                        }
                    }
                    'b_3aa68c: {
                        'b_3aa688: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V0][0] != c.g[S4][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            if t {
                                break 'b_3aa688;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S5][0]);
                                if t {
                                    break 'b_3aa68c;
                                } else {
                                    break 'b_3aa688;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        break 'b_3aa68c;
                    }
                    'b_3aa6b4: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        let t = c.g[A1][0] == 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        if t {
                            break 'b_3aa6b4;
                        } else {
                            c.cycles += 7;
                            c.g[A2][0] = 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[RA][0] = sx(0x003aa6b4);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x003aa6b4 {
                                return Err(crate::call::Unwound);
                            }
                            break 'b_3aa6b4;
                        }
                    }
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S3][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003aa650);
                        }
                        continue 'l_3aa650;
                    } else {
                        break 'b_3aa6c0;
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_v06 (0x00003aa438).

/// `cPS2ChunkResolve_v06`: resolve a PlayStation 2 chunk descriptor (version 6) to data pointers.
pub fn c_ps2_chunk_resolve_v06(c: &mut Cpu) {
    guest(c, |c| {
        'b_3aa4f4: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S2][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            let t = (c.g[V1][0] as i64) <= 0;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            if t {
                break 'b_3aa4f4;
            } else {
                c.cycles += 5;
                c.g[S4][0] = 0xffffffffffff0000;
                c.g[V0][0] = 0x500000;
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
                c.g[S3][0] = c.g[V1][0];
                c.g[S4][0] = c.g[S4][0] | 0xffff;
                'l_3aa480: loop {
                    'b_3aa4a8: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        let t = c.g[A1][0] == 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        if t {
                            break 'b_3aa4a8;
                        } else {
                            c.cycles += 7;
                            c.g[A2][0] = c.g[S1][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[RA][0] = sx(0x003aa4a8);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x003aa4a8 {
                                return Err(crate::call::Unwound);
                            }
                            break 'b_3aa4a8;
                        }
                    }
                    'b_3aa4c0: {
                        'b_3aa4bc: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V0][0] != c.g[S4][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                            if t {
                                break 'b_3aa4bc;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S5][0]);
                                if t {
                                    break 'b_3aa4c0;
                                } else {
                                    break 'b_3aa4bc;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        break 'b_3aa4c0;
                    }
                    'b_3aa4e8: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        let t = c.g[A1][0] == 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        if t {
                            break 'b_3aa4e8;
                        } else {
                            c.cycles += 7;
                            c.g[A2][0] = c.g[S1][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[RA][0] = sx(0x003aa4e8);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x003aa4e8 {
                                return Err(crate::call::Unwound);
                            }
                            break 'b_3aa4e8;
                        }
                    }
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S3][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003aa480);
                        }
                        continue 'l_3aa480;
                    } else {
                        break 'b_3aa4f4;
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cWorldCache_PointBoxDistance (0x3a9e50, worldview.cpp)

/// `c.le.s a, b`, latched in the FCC bit as the original leaves it.
#[inline]
pub(crate) fn fpu_le(c: &mut Cpu, a: crate::ps2f::FpuF32, b: crate::ps2f::FpuF32) -> bool {
    let le = a.le(b);
    crate::abi::set_fcc(c, le);
    le
}

/// One axis of the outside distance: `pos - max` when the point lies above
/// the box on that axis (the difference is not negative), else `min - pos`.
/// `x_axis` selects the x variant, whose `bmin` reload costs two cycles; the
/// y/z variants compute it in a branch-likely delay slot from the saved
/// registers. Returns the (squared if `squared`) term; leaves f0/f1 as the
/// original does.
#[inline]
pub(crate) fn point_box_axis(
    c: &mut Cpu,
    x_axis: bool,
    pos: crate::ps2f::FpuF32,
    min: crate::ps2f::FpuF32,
    bmax: crate::guest::Addr,
    bmin: crate::guest::Addr,
    squared: bool,
) -> crate::ps2f::FpuF32 {
    use crate::ps2f::FpuF32;
    c.cycles += 7;
    let max = bmax.read_fpu(c);
    crate::abi::set_fpr(c, 1, FpuF32::ZERO);
    let mut d = pos - max;
    if crate::abi::c_lt_s(c, d, FpuF32::ZERO) {
        if x_axis {
            c.cycles += 2;
            let min = bmin.read_fpu(c);
            d = min - pos;
        } else {
            d = min - pos;
        }
    }
    c.cycles += 2;
    if squared {
        d = d * d;
    }
    crate::abi::set_fpr(c, 0, d);
    c.cycles += 1;
    d
}

/// `float cWorldCache_PointBoxDistance(void* self, const Vec3* pos,
/// const Vec3* bmin, const Vec3* bmax)` (0x3a9e50): how far a point lies
/// outside an axis-aligned box, used by the world cache to rank streaming
/// nodes. Returns -1.0 when the point is inside (bounds inclusive). When it
/// is outside on exactly one axis the result is that axis' plain distance;
/// outside on two or three axes it is the Euclidean distance to the box
/// (squared terms, then `sqrt.s`).
///
/// Register leftovers reproduced: t3/t0/v1 = inside flags per axis; on the
/// inside path at = -1.0 bits; otherwise a0/t1/t2 = outside flags per axis,
/// v0 = "more than one axis outside"; f0..f7 and FCC as the original.
pub fn world_cache_point_box_distance(c: &mut Cpu) {
    use crate::abi::{set_fpr, set_reg};
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;

    let pos = Addr(r32(c, A1));
    let bmin = Addr(r32(c, A2));
    let bmax = Addr(r32(c, A3));

    // X: f1 = pos.x, f0 = bmin.x (then bmax.x).
    c.cycles += 6;
    let px = pos.read_fpu(c);
    let min_x = bmin.read_fpu(c);
    set_fpr(c, 1, px);
    set_fpr(c, 0, min_x);
    let mut in_x = fpu_le(c, min_x, px);
    if in_x {
        c.cycles += 5;
        let max_x = bmax.read_fpu(c);
        set_fpr(c, 0, max_x);
        in_x = fpu_le(c, px, max_x);
    }

    // Y: f6 = bmin.y, f4 = pos.y.
    c.cycles += 8;
    let min_y = bmin.off(4).read_fpu(c);
    let py = pos.off(4).read_fpu(c);
    set_fpr(c, 1, min_y);
    set_fpr(c, 0, py);
    set_fpr(c, 6, min_y);
    set_fpr(c, 4, py);
    let mut in_y = fpu_le(c, min_y, py);
    if in_y {
        c.cycles += 5;
        let max_y = bmax.off(4).read_fpu(c);
        set_fpr(c, 0, max_y);
        in_y = fpu_le(c, py, max_y);
    }

    // Z: f7 = bmin.z, f5 = pos.z.
    c.cycles += 8;
    let min_z = bmin.off(8).read_fpu(c);
    let pz = pos.off(8).read_fpu(c);
    set_fpr(c, 1, min_z);
    set_fpr(c, 0, pz);
    set_fpr(c, 7, min_z);
    set_fpr(c, 5, pz);
    let mut in_z = fpu_le(c, min_z, pz);
    if in_z {
        c.cycles += 5;
        let max_z = bmax.off(8).read_fpu(c);
        set_fpr(c, 0, max_z);
        in_z = fpu_le(c, pz, max_z);
    }

    set_reg(c, T3, in_x as u64);
    set_reg(c, T0, in_y as u64);
    set_reg(c, V1, in_z as u64);

    // Inside on every axis: -1.0.
    c.cycles += 2;
    if in_x {
        c.cycles += 2;
        if in_y {
            c.cycles += 2;
            if in_z {
                c.cycles += 4;
                set_reg(c, AT, 0xffff_ffff_bf80_0000);
                set_fpr(c, 0, FpuF32(0xbf80_0000));
                ret(c);
                return;
            }
        }
    }

    // Which axes to measure, and whether to combine them Euclidean-style.
    let (out_x, out_y, out_z) = (!in_x, !in_y, !in_z);
    let squared = (out_x as u32 + out_y as u32 + out_z as u32) > 1;
    c.cycles += 6 + if squared { 7 } else { 4 };
    set_fpr(c, 2, FpuF32::ZERO);
    set_reg(c, A0, out_x as u64);
    set_reg(c, T1, out_y as u64);
    set_reg(c, T2, out_z as u64);
    set_reg(c, V0, squared as u64);

    let mut sum = FpuF32::ZERO;
    c.cycles += 2;
    if out_x {
        // The x term reloads pos.x into f3.
        let px = pos.read_fpu(c);
        set_fpr(c, 3, px);
        let d = point_box_axis(c, true, px, min_x, bmax, bmin, squared);
        sum = sum + d;
        set_fpr(c, 2, sum);
    }
    c.cycles += 2;
    if out_y {
        let d = point_box_axis(c, false, py, min_y, bmax.off(4), bmin.off(4), squared);
        sum = sum + d;
        set_fpr(c, 2, sum);
    }
    c.cycles += 2;
    if out_z {
        let d = point_box_axis(c, false, pz, min_z, bmax.off(8), bmin.off(8), squared);
        sum = sum + d;
        set_fpr(c, 2, sum);
    }

    c.cycles += 2;
    if squared {
        c.cycles += 1;
        sum = sum.sqrt();
        set_fpr(c, 2, sum);
    }
    c.cycles += 2;
    set_fpr(c, 0, sum);
    ret(c);
}

pub(crate) mod ccr_release {

    pub const ENTRY: u32 = 0x003a_b498;

    pub const END: u32 = 0x003a_be40;

    /// The message jump table (.rodata, 23 entries).
    pub const TABLE: u32 = 0x0049_5010;

    pub const MESSAGES: u64 = 0x17;

    /// `self` fields: vtable, the spatial hash (world cells), a base index.
    pub const SELF_VT: u32 = 0x8;

    pub const SELF_HASH: u32 = 0xc;

    pub const SELF_BASE: u32 = 0x10;

    /// Spatial hash: update counter; eight roots of 0x14 bytes, picked by
    /// the key's signs (x: +0x50, y: +0x28, z: +0x14), node pointer at +0x10.
    pub const HASH_UPDATES: u32 = 0xa0;

    pub const ROOT_NODE: u32 = 0x10;

    /// Item fields: kind (+8); a list's count (+0x20) and first item (+0x24);
    /// an item's next (+0x64).
    pub const ITEM_KIND: u32 = 0x8;

    pub const LIST_COUNT: u32 = 0x20;

    pub const LIST_FIRST: u32 = 0x24;

    pub const ITEM_NEXT: u32 = 0x64;

    /// gp-relative globals.
    pub const GP_MISSION_MAN: u32 = 0x2a90;

    pub const GP_WP_FILES: u32 = 0x16c8;

    pub const GP_AMBIENT: u32 = 0x84c;
    // gp - 0x84c
    pub const GP_WORLD: u32 = 0x848;
    // gp - 0x848
    /// Callees.
    pub const KEY_FROM_BOX: u32 = 0x0032_8f28;

    pub const HASH_REMOVE: u32 = 0x0032_84b8;

    pub const DELETE: u32 = 0x0031_7e50;
}

/// Exit of the message switch: through 0x3abe10 (`v0 = data`) or straight
/// to the epilogue (v0 already set).
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum CcrExit {
    ResultData,
    Epilogue,
}

/// `void* func_003AB498(char* self, unsigned msg, unsigned param, char* data)`
/// (0x3ab498, named `cPS2ChunkResolve_Release` here): the world view's message
/// handler. Messages 1/3/5/6/7/8 re-file items in the spatial hash (box from
/// the item, key, `SpatialHash_Remove`, delete the root node when it is
/// empty); 9/10 forward `param >> 8` to the mission manager's vtable slot
/// 50; the others call one subsystem each (sound banks, waypoints, occlusion,
/// avalanche reset ...). Returns `data` (0 for message 14). Frame 0xc0
/// (box at 0, key at 0x20, ra 0x30, s0-s6, f20 at 0xb0).
pub fn cps2_chunk_resolve_release(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xc0 };
        c.cycles += 15;
        Frame::open(c, 0xc0);
        c.g[V0][0] = (c.g[A1][0] < ccr_release::MESSAGES) as u64;
        frame.save(c, S0, 0xa0);
        frame.save(c, S3, 0x70);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S1, 0x90);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0xb0);

        let exit = if c.g[V0][0] == 0 {
            CcrExit::ResultData
        } else {
            ccr_dispatch(c)?
        };
        if exit == CcrExit::ResultData {
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// The jump table and the message bodies.
pub(crate) fn ccr_dispatch(c: &mut Cpu) -> Result<CcrExit, crate::call::Unwound> {
    use ccr_release as k;
    c.cycles += 7;
    c.g[V0][0] = sx(k::TABLE);
    c.g[V1][0] = sx(r32(c, A1) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(k::TABLE));
    c.g[A0][0] = c.lw(r32(c, V1));
    match r32(c, A0) {
        // 0, 2, 4, 12, 16: nothing.
        0x003a_be10 => Ok(CcrExit::ResultData),
        // 1: re-file `data` (box at +0x158) as kind 1.
        0x003a_b4f0 => {
            c.cycles += 46;
            ccr_std_update(c, 0x158, 1, CCR_SCAN_1)
        }
        // 3: box at +0x60, kind 0.
        0x003a_b664 => {
            c.cycles += 46;
            ccr_std_update(c, 0x60, 0, CCR_SCAN_3)
        }
        // 5: kind word 7, box at +0x68, kind 2.
        0x003a_bbb0 => {
            c.cycles += 48;
            c.g[V0][0] = 7;
            c.sw(r32(c, S3).wrapping_add(k::ITEM_KIND), 7);
            ccr_std_update(c, 0x68, 2, CCR_SCAN_5)
        }
        // 6: if self->vt[22](data): kind word 6, box at +0x44, kind 2.
        0x003a_b918 => {
            c.cycles += 8;
            c.g[V1][0] = c.lw(r32(c, S4).wrapping_add(k::SELF_VT));
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0xb0));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xb4));
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x003a_b930)?;
            let refile = c.g[V0][0] != 0;
            c.g[V0][0] = 6;
            if !refile {
                return Ok(CcrExit::ResultData);
            }
            c.cycles += 47;
            c.sw(r32(c, S3).wrapping_add(k::ITEM_KIND), 6);
            ccr_std_update(c, 0x44, 2, CCR_SCAN_6)
        }
        // 7: kind word 8, box at +0x28, kind 2.
        0x003a_ba70 => {
            c.cycles += 48;
            c.g[V0][0] = 8;
            c.sw(r32(c, S3).wrapping_add(k::ITEM_KIND), 8);
            ccr_std_update(c, 0x28, 2, CCR_SCAN_7)
        }
        // 8: every item of the list.
        0x003a_b798 => ccr_list_update(c),
        // 9: mission manager vt[50](param >> 8).
        0x003a_b628 => {
            c.cycles += 4;
            c.g[A2][0] = c.lw(r32(c, GP).wrapping_add(k::GP_MISSION_MAN));
            c.g[A1][0] = sx(r32(c, S0) >> 8);
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x10d8));
            ccr_mission_call(c)
        }
        // 10: mission manager vt[50]((param >> 8) + self->base).
        0x003a_b638 => {
            c.cycles += 5;
            c.g[A2][0] = c.lw(r32(c, GP).wrapping_add(k::GP_MISSION_MAN));
            c.g[V1][0] = sx(r32(c, S0) >> 8);
            c.g[A1][0] = c.lw(r32(c, S4).wrapping_add(k::SELF_BASE));
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x10d8));
            c.g[A1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
            ccr_mission_call(c)
        }
        // 13: func_002B6B30(func_0028B1C8()->+0x4c, data, param).
        0x003a_bd04 => {
            c.cycles += 8;
            call_ptr(c, 0x0028_b1c8, 0x003a_bd0c)?;
            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x4c));
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x002b_6b30, 0x003a_bd1c)?;
            Ok(ccr_done(c))
        }
        // 17: func_0016CF18(param).
        0x003a_bd24 => {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0016_cf18, 0x003a_bd2c)?;
            Ok(ccr_done(c))
        }
        // 15: WPFile_Unload(&files->table[param & 0xff]) if loaded.
        0x003a_bd34 => {
            c.cycles += 3;
            c.g[A3][0] = c.lw(r32(c, GP).wrapping_add(k::GP_WP_FILES));
            c.g[A1][0] = c.g[S0][0] & 0xff;
            if c.g[A3][0] == 0 {
                return Ok(CcrExit::ResultData);
            }
            c.cycles += 8;
            c.g[V0][0] = 0xc;
            c.mult(0, A1, A1, V0);
            c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(0x8));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x4));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            call_ptr(c, 0x002b_b5a8, 0x003a_bd58)?;
            Ok(ccr_done(c))
        }
        // 18: func_00278A58(global, param & 0xff).
        0x003a_bd60 => {
            c.cycles += 5;
            c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(k::GP_AMBIENT));
            c.g[A1][0] = c.g[S0][0] & 0xff;
            call_ptr(c, 0x0027_8a58, 0x003a_bd6c)?;
            Ok(ccr_done(c))
        }
        // 20: SSXAudioSystem_UnloadMemoryBank(func_0028B180(), param, data).
        0x003a_bd74 => {
            c.cycles += 8;
            call_ptr(c, 0x0028_b180, 0x003a_bd7c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S3][0];
            call_ptr(c, 0x0028_6d18, 0x003a_bd8c)?;
            Ok(ccr_done(c))
        }
        // 19: the BE's interface 10, stub(param & 0xff).
        0x003a_bd94 => {
            c.cycles += 10;
            call_ptr(c, 0x0014_dc80, 0x003a_bd9c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0014_dd58, 0x003a_bda8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0] & 0xff;
            call_ptr(c, 0x0015_47e0, 0x003a_bdb4)?;
            Ok(ccr_done(c))
        }
        // 21: func_00210608(data).
        0x003a_bdbc => {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0021_0608, 0x003a_bdc4)?;
            Ok(ccr_done(c))
        }
        // 11: func_00242570(GameModule_GetOcclusionMan(), data).
        0x003a_bdcc => {
            c.cycles += 7;
            Callee {
                addr: 0x003a_d198,
                f: ee::s13::GameModule_GetOcclusionMan,
            }
            .call(c, 0x003a_bdd4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x0024_2570, 0x003a_bde0)?;
            Ok(ccr_done(c))
        }
        // 14: func_0012A490(world->+0x84->+0xc, param & 0xff); returns 0.
        0x003a_bde8 => {
            c.cycles += 8;
            c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(k::GP_WORLD));
            c.g[A1][0] = c.g[S0][0] & 0xff;
            c.g[S3][0] = 0;
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x84));
            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0xc));
            call_ptr(c, 0x0012_a490, 0x003a_be00)?;
            Ok(ccr_done(c))
        }
        // 22: cAvalanche_reset().
        0x003a_be08 => {
            c.cycles += 2;
            call_ptr(c, 0x002d_9b40, 0x003a_be10)?;
            Ok(CcrExit::ResultData)
        }
        pc => {
            c.jump_out(pc, k::ENTRY, k::END);
            Err(crate::call::Unwound)
        }
    }
}

/// `v0 = data` in the delay slot of the branch to the epilogue.
pub(crate) fn ccr_done(c: &mut Cpu) -> CcrExit {
    c.g[V0][0] = c.g[S3][0];
    CcrExit::Epilogue
}

/// Messages 9/10 (0x3ab64c, a2 = the manager, a1 = the argument, v0 = its
/// vtable): `vt[50].fn(mgr + vt[50].delta, a1)`.
pub(crate) fn ccr_mission_call(c: &mut Cpu) -> Result<CcrExit, crate::call::Unwound> {
    c.cycles += 6;
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x190));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x194));
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x003a_b65c)?;
    Ok(ccr_done(c))
}

/// How one copy of the inlined "is the node empty" scan differs: the cycles
/// after the eighth child, the item loop's resume points, whether it is the
/// message-5 copy (different tail), and the call return addresses.
#[derive(Clone, Copy)]
pub(crate) struct CcrScan {
    key_ret: u32,
    remove_ret: u32,
    after_children: u64,
    items_head: u32,
    msg5: bool,
}

pub(crate) const CCR_SCAN_1: CcrScan = CcrScan {
    key_ret: 0x003a_b54c,
    remove_ret: 0x003a_b598,
    after_children: 3,
    items_head: 0x003a_b608,
    msg5: false,
};

pub(crate) const CCR_SCAN_3: CcrScan = CcrScan {
    key_ret: 0x003a_b6c0,
    remove_ret: 0x003a_b70c,
    after_children: 2,
    items_head: 0x003a_b778,
    msg5: false,
};

pub(crate) const CCR_SCAN_6: CcrScan = CcrScan {
    key_ret: 0x003a_b998,
    remove_ret: 0x003a_b9e4,
    after_children: 2,
    items_head: 0x003a_ba50,
    msg5: false,
};

pub(crate) const CCR_SCAN_7: CcrScan = CcrScan {
    key_ret: 0x003a_bad4,
    remove_ret: 0x003a_bb20,
    after_children: 3,
    items_head: 0x003a_bb90,
    msg5: false,
};

pub(crate) const CCR_SCAN_5: CcrScan = CcrScan {
    key_ret: 0x003a_bc14,
    remove_ret: 0x003a_bc60,
    after_children: 1,
    items_head: 0x003a_bcd0,
    msg5: true,
};

/// Message 5's "not empty" block (event resume point).
pub(crate) const CCR_NOT_EMPTY: u32 = 0x003a_bcc4;

/// The item's box (`base + off`: min xyz, max xyz) on the stack with w =
/// 1.0 from FPU register `one`; a0 = the key's slot (sp + 0x20), a1 = the box.
pub(crate) fn ccr_fill_box(c: &mut Cpu, base: u32, off: u32, one: usize) {
    let s = sp(c);
    let src = base.wrapping_add(off);
    c.lwc1(0, src);
    c.g[A0][0] = sx(s.wrapping_add(0x20));
    c.g[A1][0] = c.g[SP][0];
    c.swc1(0, s);
    c.lwc1(0, src.wrapping_add(0x4));
    c.swc1(0, s + 0x4);
    c.lwc1(1, src.wrapping_add(0x8));
    c.swc1(one, s + 0xc);
    c.swc1(1, s + 0x8);
    c.lwc1(0, src.wrapping_add(0xc));
    c.swc1(0, s + 0x10);
    c.lwc1(1, src.wrapping_add(0x10));
    c.swc1(1, s + 0x14);
    c.lwc1(0, src.wrapping_add(0x14));
    c.swc1(one, s + 0x1c);
    c.swc1(0, s + 0x18);
}

/// The inlined `update_B498` up to `SpatialHash_Remove` (box already on the
/// stack): count the update, key from the box, the root by the key's signs
/// (s0), remove `item` (kind `kind`) from its node. Leaves v1 = the node and
/// v0 = its first child.
pub(crate) fn ccr_remove(
    c: &mut Cpu,
    item: u64,
    kind: u64,
    key_ret: u32,
    remove_ret: u32,
) -> crate::call::Flow {
    use ccr_release as k;
    c.g[S0][0] = c.lw(r32(c, S4).wrapping_add(k::SELF_HASH));
    let updates = sx((c.lw(r32(c, S0).wrapping_add(k::HASH_UPDATES)) as u32).wrapping_add(1));
    c.g[V0][0] = updates;
    c.sw(r32(c, S0).wrapping_add(k::HASH_UPDATES), updates);
    call_ptr(c, k::KEY_FROM_BOX, key_ret)?;
    let s = sp(c);
    let kx = c.lw(s + 0x24) as i64;
    let ky = c.lw(s + 0x28) as i64;
    let kz = c.lw(s + 0x2c) as i64;
    let hash = r32(c, S0);
    let rx = if -1 < kx {
        hash
    } else {
        hash.wrapping_add(0x50)
    };
    let ry = if -1 < ky { rx } else { rx.wrapping_add(0x28) };
    let root = if -1 < kz { ry } else { ry.wrapping_add(0x14) };
    c.g[V1][0] = sx(rx);
    c.g[V0][0] = sx(ry);
    c.g[S0][0] = sx(root);
    c.g[A0][0] = c.lw(root.wrapping_add(k::ROOT_NODE));
    c.g[A1][0] = kind;
    c.g[A2][0] = item;
    c.g[A3][0] = sx(s.wrapping_add(0x20));
    c.g[T0][0] = c.g[S0][0];
    call_ptr(c, k::HASH_REMOVE, remove_ret)?;
    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(k::ROOT_NODE));
    c.g[V0][0] = c.lw(r32(c, V1));
    Ok(())
}

/// Messages 1/3/5/6/7 after the block's charge: box from `data + off`,
/// remove as `kind`, then delete the root's node if it is empty.
pub(crate) fn ccr_std_update(
    c: &mut Cpu,
    off: u32,
    kind: u64,
    scan: CcrScan,
) -> Result<CcrExit, crate::call::Unwound> {
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 2);
    let data = r32(c, S3);
    ccr_fill_box(c, data, off, 2);
    let item = c.g[S3][0];
    ccr_remove(c, item, kind, scan.key_ret, scan.remove_ret)?;

    // Empty: no children (+0..+0x1c) and no items (+0x20..+0x28).
    let empty = 'scan: {
        let node = r32(c, V1);
        for i in 0..8u32 {
            if i > 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(node.wrapping_add(4 * i));
            }
            let child = c.g[V0][0] != 0;
            if scan.msg5 && i == 7 {
                if child {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'scan false;
                }
                c.g[V1][0] = sx(node.wrapping_add(0x20));
                break;
            }
            c.g[V0][0] = 0;
            if child {
                break 'scan false;
            }
        }
        c.cycles += scan.after_children;
        if !scan.msg5 {
            c.g[V1][0] = sx(node.wrapping_add(0x20));
        }
        c.g[A0][0] = 0;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
            if c.g[V0][0] != 0 {
                if scan.msg5 && event_due(c) {
                    deliver_event_here(c, CCR_NOT_EMPTY);
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'scan false;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, scan.items_head);
            }
        }
        c.cycles += if scan.msg5 { 1 } else { 2 };
        c.g[V0][0] = 1;
        true
    };

    // 0x3abcec: delete the empty node.
    c.cycles += 2;
    c.g[V0][0] = c.g[S3][0];
    if !empty {
        return Ok(CcrExit::Epilogue);
    }
    c.cycles += 4;
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(ccr_release::ROOT_NODE));
    call_ptr(c, ccr_release::DELETE, 0x003a_bcfc)?;
    c.sw(r32(c, S0).wrapping_add(ccr_release::ROOT_NODE), 0);
    Ok(CcrExit::ResultData)
}

/// Message 8 (0x3ab798): for each of the list's items (count `data+0x20`,
/// first `data+0x24`, next `+0x64`): kind word 1, box at +0x6c, remove as
/// kind 2, delete the root's node if it is empty.
pub(crate) fn ccr_list_update(c: &mut Cpu) -> Result<CcrExit, crate::call::Unwound> {
    use ccr_release as k;
    c.cycles += 4;
    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(k::LIST_COUNT));
    c.g[S2][0] = 0;
    c.g[S1][0] = c.lw(r32(c, S3).wrapping_add(k::LIST_FIRST));
    if (c.g[V0][0] as i64) <= 0 {
        return Ok(CcrExit::ResultData);
    }
    c.cycles += 4;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 20);
    c.g[S6][0] = 1;
    c.g[S5][0] = u64::MAX;
    loop {
        // 0x3ab7b8
        c.cycles += 44;
        let item = r32(c, S1);
        c.sw(item.wrapping_add(k::ITEM_KIND), c.g[S6][0]);
        ccr_fill_box(c, item, 0x6c, 20);
        let item = c.g[S1][0];
        ccr_remove(c, item, 2, 0x003a_b810, 0x003a_b858)?;

        let empty = 'scan: {
            let node = r32(c, V1);
            for i in 0..8u32 {
                if i > 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(node.wrapping_add(4 * i));
                }
                let child = c.g[V0][0] != 0;
                if i == 6 {
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                } else if child && i < 6 {
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                }
                if child {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'scan false;
                }
            }
            c.cycles += 3;
            c.g[V1][0] = sx(node.wrapping_add(0x20));
            c.g[A0][0] = 0;
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, V1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_b8bc);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'scan false;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_b8d0);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
            true
        };

        // 0x3ab8ec
        c.cycles += 2;
        if empty {
            c.cycles += 4;
            c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(k::ROOT_NODE));
            call_ptr(c, k::DELETE, 0x003a_b8fc)?;
            c.sw(r32(c, S0).wrapping_add(k::ROOT_NODE), 0);
        }
        c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(k::LIST_COUNT));
        // 0x3ab904
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[S1][0] = c.lw(r32(c, S1).wrapping_add(k::ITEM_NEXT));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_b7b8);
        }
    }
    c.cycles += 2;
    Ok(ccr_done(c))
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_Resolve (0x3aae68)

pub(crate) const CRR_ENTRY: u32 = 0x003a_ae68;

pub(crate) const CRR_END: u32 = 0x003a_b494;

/// The message switch's jump table (`.rodata`, messages 0..22).
pub(crate) const CRR_JUMP_TABLE: u32 = 0x0049_4fb0;

pub(crate) const CRR_MSG_COUNT: u64 = 0x17;

/// Stream names handed to the world's stream handler (messages 9 and 10).
pub(crate) const CRR_STRM_TEX: u32 = 0x0049_4f88;

pub(crate) const CRR_STRM_LPG: u32 = 0x0049_4f98;

/// Resolver fields: its vtable, the spatial hash set (eight hashes of 0x14
/// bytes, picked by the signs of the key), the section's base index.
pub(crate) const CRR_VTABLE: u32 = 0x8;

pub(crate) const CRR_HASHES: u32 = 0xc;

pub(crate) const CRR_BASE_INDEX: u32 = 0x10;

/// Resolver vtable slots (`{delta, fn}` pairs).
pub(crate) const CRR_VT_REGISTER_B: u32 = 0x28;

// message 2
pub(crate) const CRR_VT_REGISTER_A: u32 = 0x48;

// message 0
pub(crate) const CRR_VT_INSTANCE: u32 = 0x68;

// message 3
pub(crate) const CRR_VT_REGISTER_C: u32 = 0x70;

// message 4
pub(crate) const CRR_VT_OCCLUDER: u32 = 0x88;

// message 5
pub(crate) const CRR_VT_PATCH_LIST: u32 = 0x90;

// message 8
pub(crate) const CRR_VT_GENERIC: u32 = 0x98;

// message 12
pub(crate) const CRR_VT_VOLUME: u32 = 0xa0;

// message 7
pub(crate) const CRR_VT_VISIBLE: u32 = 0xa8;

// message 6

/// Chunk fields: the spatial kind (+8), a patch list's count and head
/// (+0x20/+0x24, linked through +0x64), and the bounding boxes per chunk
/// type (min xyz then max xyz, six floats).
pub(crate) const CRR_OBJ_KIND: u32 = 0x8;

pub(crate) const CRR_OBJ_COUNT: u32 = 0x20;

pub(crate) const CRR_OBJ_LIST: u32 = 0x24;

pub(crate) const CRR_NODE_NEXT: u32 = 0x64;

pub(crate) const CRR_NODE_BOX: u32 = 0x6c;

pub(crate) const CRR_MODEL_COUNTER: u32 = 0x1a2;

pub(crate) const CRR_MODEL_BOX: u32 = 0x158;

pub(crate) const CRR_INSTANCE_BOX: u32 = 0x60;

pub(crate) const CRR_VISIBLE_BOX: u32 = 0x44;

pub(crate) const CRR_VOLUME_BOX: u32 = 0x28;

pub(crate) const CRR_OCCLUDER_BOX: u32 = 0x68;

/// Globals: `gp+0x2a90` the world view (its vtable at +0x10d8, the stream
/// handler in slot 0x170), `gp+0x16c8` the script resources (`D_004A47B8`),
/// `gp-0x84c` the locator table owner, `gp-0x848` the world.
pub(crate) const CRR_GP_WORLD_VIEW: u32 = 0x2a90;

pub(crate) const CRR_GP_SCRIPT_RES: u32 = 0x16c8;

pub(crate) const CRR_GP_LOCATORS_NEG: u32 = 0x84c;

pub(crate) const CRR_GP_WORLD_NEG: u32 = 0x848;

pub(crate) const CRR_SPATIAL_KEY: u32 = 0x0032_8f28;

// SpatialKey_FromAABB
pub(crate) const CRR_SPATIAL_INSERT: u32 = 0x0032_8c20;

// SpatialHash_Insert
pub(crate) const CRR_AUDIO_GAME_MODULE: u32 = 0x0028_b1c8;

// SSXAudioSystem_GetGameModule
pub(crate) const CRR_WORLD_SOUND_LOADED: u32 = 0x002b_6b08;

// cWorldSound_OnSectionLoaded
pub(crate) const CRR_CAMERA_RELOAD: u32 = 0x0016_cec8;

// CameraTriggerMan_ReloadIf_Global
pub(crate) const CRR_PAINTER_SET: u32 = 0x002b_b348;

// cWorldPainterMan_SetFileData
pub(crate) const CRR_SCRIPT_TABLE: u32 = 0x0023_0430;

// GameModule_CreateSectionScriptTable
pub(crate) const CRR_LOCATORS_SET: u32 = 0x0027_8a10;

// ScriptEngine_SetLocatorTable
pub(crate) const CRR_AUDIO_INSTANCE: u32 = 0x0028_b180;

// SSXAudioSystem_GetInstance
pub(crate) const CRR_AUDIO_LOAD_BANK: u32 = 0x0028_6ca8;

// SSXAudioSystem_LoadMemoryBank
pub(crate) const CRR_GET_BE: u32 = 0x0014_dc80;

// cBE_getBE
pub(crate) const CRR_BE_INTERFACE: u32 = 0x0014_dd58;

// BELibrary_GetInterface
pub(crate) const CRR_MISSION_STUB: u32 = 0x0015_47d8;

// BEMissionInt_EmptyStub_47d8
pub(crate) const CRR_RACE_PATH_SET: u32 = 0x0021_05d0;

// RaceProgress_SetPathData
pub(crate) const CRR_ADD_OCCLUDER: u32 = 0x0024_2540;

// OcclusionMan_AddOccluder
pub(crate) const CRR_TRACK_PATHS: u32 = 0x0012_a340;

// AI_SetTrackPathsBuffer
pub(crate) const CRR_MEM_FREE: u32 = 0x0031_7e98;

// EA_MemFree
pub(crate) const CRR_AVALANCHE_SET: u32 = 0x002d_9fb8;

// Avalanche_SetHistoryData

pub(crate) const CRR_SCRIPT_CACHE_SET: Callee = Callee {
    addr: 0x003a_ca10,
    f: ee::s13::cWScriptCache_SetData,
};

pub(crate) const CRR_APP_GAME_MODULE: Callee = Callee {
    addr: 0x003a_d188,
    f: ee::s13::SSXApp_GetGameModule,
};

pub(crate) const CRR_OCCLUSION_MAN: Callee = Callee {
    addr: 0x003a_d198,
    f: ee::s13::GameModule_GetOcclusionMan,
};

/// How a message handler leaves: the shared tails of the original.
pub(crate) enum CrrExit {
    /// 0x3ab464: `v0 = obj`, reload s0, epilogue.
    Plain,
    /// 0x3ab468: `v0` already set, reload s0, epilogue.
    Done,
    /// 0x3ab46c: s0 already reloaded, epilogue.
    Epilogue,
    /// The jump table pointed at the entry: run the function again.
    Restart,
}

/// `char* cPS2ChunkResolve::resolve(eBxStreamDataType type /*a1*/,
/// tResourceID rid /*a2*/, char* data /*a3*/)` (0x3aae68): hand a streamed
/// chunk to whoever owns its type, and return `data`.
///
/// - 1 (model): bump the model's counter (+0x1a2) by the section's base
///   index and insert it into the spatial hash (kind 1).
/// - 0, 2, 4: register through a resolver virtual (+0x48, +0x28, +0x70).
/// - 3 (instance): resolver virtual +0x68, then insert (kind 0).
/// - 5, 6, 7: resolver virtual (+0x88, +0xa8 only inserting when it returns
///   non-zero, +0xa0), set the chunk's kind to 7, 6, 8 and insert it (kind 2).
/// - 8: resolver virtual +0x90, then mark every patch in the chunk's list
///   kind 1 and insert it (kind 2).
/// - 9, 10: the world view's stream handler with "strm_tex"/"strm_lpg" and
///   the texture index (`rid >> 8`, plus the base index for 10).
/// - 11-22: occluders, the generic resolver virtual, world sounds, track
///   paths (freeing the data when rejected), painter files, script tables,
///   camera triggers, locators, mission data, sound banks, race paths and
///   avalanche history.
///
/// Spatial inserts build the box at `sp` (w = 1), get its key into `sp+0x20`
/// (`SpatialKey_FromAABB`) and pick one of the eight hashes at
/// `resolver+0xc` by the signs of the key's x, y, z.
pub fn c_ps2_chunk_resolve_resolve(c: &mut Cpu) {
    if c.pc != CRR_ENTRY {
        return ee::s13::f_003aae68__entries(c);
    }
    c.cycles += 15;
    let frame = Frame::open(c, 0xc0);
    c.g[V0][0] = (c.g[A1][0] < CRR_MSG_COUNT) as u64;
    frame.save(c, S0, 0xa0);
    frame.save(c, S2, 0x80);
    c.g[S0][0] = c.g[A2][0];
    frame.save(c, S3, 0x70);
    c.g[S2][0] = c.g[A3][0];
    frame.save(c, S1, 0x90);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    frame.save(c, S6, 0x40);
    frame.save_ra(c, 0x30);
    frame.save_fpr(c, 20, 0xb0);
    let exit = if c.g[V0][0] == 0 {
        Ok(CrrExit::Plain)
    } else {
        crr_dispatch(c)
    };
    let exit = match exit {
        Ok(CrrExit::Restart) => {
            c.pc = CRR_ENTRY;
            return c_ps2_chunk_resolve_resolve(c);
        }
        Ok(e) => e,
        Err(Unwound) => return,
    };
    if let CrrExit::Plain = exit {
        c.cycles += 1;
        c.g[V0][0] = c.g[S2][0];
    }
    if !matches!(exit, CrrExit::Epilogue) {
        c.cycles += 1;
        frame.restore(c, S0, 0xa0);
    }
    c.cycles += 10;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore_ra(c, 0x30);
    frame.restore_fpr(c, 20, 0xb0);
    frame.close(c);
}

/// The jump table dispatch (0x3aaea4) and the handlers.
pub(crate) fn crr_dispatch(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 7;
    c.g[V0][0] = 0x0049_0000;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx(CRR_JUMP_TABLE);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(CRR_JUMP_TABLE));
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    let target = c.g[A0][0] as u32;
    match target {
        CRR_ENTRY => {
            if event_due(c) {
                deliver_event_here(c, CRR_ENTRY);
            }
            Ok(CrrExit::Restart)
        }
        0x003a_aec0 => crr_model(c),
        0x003a_af5c => crr_stream(c, false),
        0x003a_af88 => crr_stream(c, true),
        0x003a_afc8 => crr_instance(c),
        0x003a_b06c => {
            c.cycles += 5;
            crr_vmethod(c, CRR_VT_REGISTER_B);
            crr_vcall_obj(c)
        }
        0x003a_b080 => {
            c.cycles += 5;
            crr_vmethod(c, CRR_VT_REGISTER_A);
            crr_vcall_obj(c)
        }
        0x003a_b094 => crr_visible(c),
        0x003a_b0dc => crr_volume(c),
        0x003a_b120 => crr_patch_list(c),
        0x003a_b200 => crr_occluder_box(c),
        0x003a_b2bc => {
            c.cycles += 4;
            crr_vmethod(c, CRR_VT_REGISTER_C);
            crr_vcall_obj(c)
        }
        0x003a_b2dc => crr_generic(c),
        0x003a_b300 => crr_world_sound(c),
        0x003a_b320 => crr_camera_triggers(c),
        0x003a_b334 => crr_painter(c),
        0x003a_b354 => crr_script_table(c),
        0x003a_b38c => crr_locators(c),
        0x003a_b3a4 => crr_sound_bank(c),
        0x003a_b3c4 => crr_mission(c),
        0x003a_b3f4 => {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, CRR_RACE_PATH_SET, 0x003a_b3fc)?;
            c.g[V0][0] = c.g[S2][0];
            Ok(CrrExit::Done)
        }
        0x003a_b404 => crr_occluder(c),
        0x003a_b420 => crr_track_paths(c),
        0x003a_b45c => {
            c.cycles += 2;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, CRR_AVALANCHE_SET, 0x003a_b464)?;
            Ok(CrrExit::Plain)
        }
        _ => {
            c.jump_out(target, CRR_ENTRY, CRR_END);
            Err(Unwound)
        }
    }
}

/// Read resolver vtable slot `slot` (`v1 = vtable`, `a0 = delta`,
/// `v0 = fn`) with `a1 = data`.
pub(crate) fn crr_vmethod(c: &mut Cpu, slot: u32) {
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(CRR_VTABLE)).lw(c);
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(slot + 4)).lw(c);
}

/// `jalr v0` with `a0 = resolver + delta`, returning to `ret`.
pub(crate) fn crr_vcall(c: &mut Cpu, ret: u32) -> Flow {
    let f = c.g[V0][0] as u32;
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
    call_ptr(c, f, ret)
}

/// 0x3ab2cc: the virtual alone, return `data`.
pub(crate) fn crr_vcall_obj(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 4;
    crr_vcall(c, 0x003a_b2d4)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Load the chunk's box at `data+off` into f5, f0..f4; `a0 = sp+0x20` (the
/// key), `a1 = sp` (the box).
pub(crate) fn crr_load_box(c: &mut Cpu, off: u32) {
    let obj = r32(c, S2).wrapping_add(off);
    c.lwc1(5, obj);
    c.g[A0][0] = sx(sp(c).wrapping_add(0x20));
    c.lwc1(0, obj.wrapping_add(0x4));
    c.g[A1][0] = c.g[SP][0];
    c.lwc1(1, obj.wrapping_add(0x8));
    c.lwc1(2, obj.wrapping_add(0xc));
    c.lwc1(3, obj.wrapping_add(0x10));
    c.lwc1(4, obj.wrapping_add(0x14));
}

/// Store the box at `sp` as `{min, 1}, {max, 1}`, `s0 = resolver->hashes`,
/// and get its key (`SpatialKey_FromAABB(sp+0x20, sp)`), returning to `ret`.
pub(crate) fn crr_box_key(c: &mut Cpu, ret: u32) -> Flow {
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 6);
    c.g[S0][0] = Addr(r32(c, S3).wrapping_add(CRR_HASHES)).lw(c);
    let s = sp(c);
    c.swc1(5, s);
    c.swc1(0, s.wrapping_add(0x4));
    c.swc1(1, s.wrapping_add(0x8));
    c.swc1(2, s.wrapping_add(0x10));
    c.swc1(3, s.wrapping_add(0x14));
    c.swc1(4, s.wrapping_add(0x18));
    c.swc1(6, s.wrapping_add(0x1c));
    c.swc1(6, s.wrapping_add(0xc));
    call_ptr(c, CRR_SPATIAL_KEY, ret)
}

/// Pick the hash by the key's signs (x >= 0: +0, else +0x50; y: +0 or
/// +0x28; z: +0 or +0x14) and insert the chunk with `kind`
/// (`SpatialHash_Insert(hash, kind, data, sp+0x20)`, 0x3ab2a8).
pub(crate) fn crr_insert(c: &mut Cpu, kind: u64) -> Result<CrrExit, Unwound> {
    let s = sp(c);
    c.g[V0][0] = Addr(s.wrapping_add(0x24)).lw(c);
    c.g[A3][0] = u64::MAX;
    c.g[V1][0] = sx(r32(c, S0).wrapping_add(0x50));
    c.g[A1][0] = Addr(s.wrapping_add(0x28)).lw(c);
    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A2][0] = Addr(s.wrapping_add(0x2c)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[S0][0];
    }
    c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(0x28));
    c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
    if c.g[A1][0] != 0 {
        c.g[A0][0] = c.g[V1][0];
    }
    c.g[A2][0] = c.g[S2][0];
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x14));
    c.g[A1][0] = kind;
    c.cycles += 5;
    if c.g[A3][0] == 0 {
        c.g[A0][0] = c.g[V0][0];
    }
    c.g[A3][0] = sx(s.wrapping_add(0x20));
    call_ptr(c, CRR_SPATIAL_INSERT, 0x003a_b2b4)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 1 (0x3aaec0): a model.
pub(crate) fn crr_model(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 39;
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(CRR_BASE_INDEX)).lhu(c);
    let counter = Addr(r32(c, S2).wrapping_add(CRR_MODEL_COUNTER));
    c.g[V0][0] = counter.lhu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    let bumped = c.g[V0][0];
    counter.sh(c, bumped);
    crr_load_box(c, CRR_MODEL_BOX);
    crr_box_key(c, 0x003a_af20)?;
    crr_insert(c, 1)
}

/// Messages 9 and 10 (0x3aaf5c, 0x3aaf88): the world view's stream handler
/// `(view + delta, data, name, 0, 0, index)`.
pub(crate) fn crr_stream(c: &mut Cpu, lightmaps: bool) -> Result<CrrExit, Unwound> {
    let view_at = r32(c, GP).wrapping_add(CRR_GP_WORLD_VIEW);
    if lightmaps {
        c.cycles += 12;
        c.g[T2][0] = Addr(view_at).lw(c);
        c.g[V0][0] = sx(r32(c, S0) >> 8);
        c.g[T1][0] = Addr(r32(c, S3).wrapping_add(CRR_BASE_INDEX)).lw(c);
        c.g[V1][0] = Addr(r32(c, T2).wrapping_add(0x10d8)).lw(c);
        c.g[A2][0] = sx(CRR_STRM_LPG);
        c.g[T1][0] = sx(r32(c, V0).wrapping_add(r32(c, T1)));
    } else {
        c.cycles += 11;
        c.g[T2][0] = Addr(view_at).lw(c);
        c.g[A2][0] = sx(CRR_STRM_TEX);
        c.g[T1][0] = sx(r32(c, S0) >> 8);
        c.g[V1][0] = Addr(r32(c, T2).wrapping_add(0x10d8)).lw(c);
    }
    c.g[A1][0] = c.g[S2][0];
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x170)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x174)).lw(c);
    c.cycles += 4;
    let f = c.g[V0][0] as u32;
    c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
    call_ptr(c, f, 0x003a_afc0)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 3 (0x3aafc8): an instance.
pub(crate) fn crr_instance(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 41;
    crr_vmethod(c, CRR_VT_INSTANCE);
    crr_vcall(c, 0x003a_afe0)?;
    crr_load_box(c, CRR_INSTANCE_BOX);
    crr_box_key(c, 0x003a_b030)?;
    crr_insert(c, 0)
}

/// 0x3ab240: key and insert with kind 2.
pub(crate) fn crr_insert_kind2(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 26;
    crr_box_key(c, 0x003a_b270)?;
    crr_insert(c, 2)
}

/// Message 6 (0x3ab094): inserted (kind 6) only when the virtual accepts it.
pub(crate) fn crr_visible(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 8;
    crr_vmethod(c, CRR_VT_VISIBLE);
    crr_vcall(c, 0x003a_b0ac)?;
    let accepted = c.g[V0][0] != 0;
    c.g[V0][0] = 6;
    if !accepted {
        return Ok(CrrExit::Plain);
    }
    c.cycles += 10;
    let kind = c.g[V0][0];
    Addr(r32(c, S2).wrapping_add(CRR_OBJ_KIND)).sw(c, kind);
    crr_load_box(c, CRR_VISIBLE_BOX);
    crr_insert_kind2(c)
}

/// Message 7 (0x3ab0dc): a volume (kind 8).
pub(crate) fn crr_volume(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 17;
    crr_vmethod(c, CRR_VT_VOLUME);
    crr_vcall(c, 0x003a_b0f4)?;
    c.g[V1][0] = 8;
    Addr(r32(c, S2).wrapping_add(CRR_OBJ_KIND)).sw(c, 8);
    crr_load_box(c, CRR_VOLUME_BOX);
    crr_insert_kind2(c)
}

/// Message 5 (0x3ab200): an occluder box (kind 7).
pub(crate) fn crr_occluder_box(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 16;
    crr_vmethod(c, CRR_VT_OCCLUDER);
    crr_vcall(c, 0x003a_b218)?;
    c.g[V1][0] = 7;
    Addr(r32(c, S2).wrapping_add(CRR_OBJ_KIND)).sw(c, 7);
    crr_load_box(c, CRR_OCCLUDER_BOX);
    crr_insert_kind2(c)
}

/// Message 8 (0x3ab120): a patch list; every patch gets kind 1 and is
/// inserted (s1 = patch, s4 = index, s5 = -1, s6 = 1, f20 = 1.0).
pub(crate) fn crr_patch_list(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 10;
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(CRR_VTABLE)).lw(c);
    c.g[A1][0] = c.g[S2][0];
    c.g[S4][0] = 0;
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(CRR_VT_PATCH_LIST)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(CRR_VT_PATCH_LIST + 4)).lw(c);
    crr_vcall(c, 0x003a_b13c)?;
    c.g[V1][0] = Addr(r32(c, S2).wrapping_add(CRR_OBJ_COUNT)).lw(c);
    c.g[S1][0] = Addr(r32(c, S2).wrapping_add(CRR_OBJ_LIST)).lw(c);
    if (c.g[V1][0] as i64) <= 0 {
        return Ok(CrrExit::Plain);
    }
    c.cycles += 4;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 20);
    c.g[S6][0] = 1;
    c.g[S5][0] = u64::MAX;
    loop {
        c.cycles += 40;
        let node = r32(c, S1);
        let s = sp(c);
        c.lwc1(0, node.wrapping_add(CRR_NODE_BOX));
        c.g[A0][0] = sx(s.wrapping_add(0x20));
        let kind = c.g[S6][0];
        Addr(node.wrapping_add(CRR_OBJ_KIND)).sw(c, kind);
        c.g[A1][0] = c.g[SP][0];
        c.swc1(0, s);
        c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
        c.lwc1(0, node.wrapping_add(CRR_NODE_BOX + 0x4));
        c.g[S0][0] = Addr(r32(c, S3).wrapping_add(CRR_HASHES)).lw(c);
        c.swc1(0, s.wrapping_add(0x4));
        c.lwc1(1, node.wrapping_add(CRR_NODE_BOX + 0x8));
        c.swc1(20, s.wrapping_add(0xc));
        c.swc1(1, s.wrapping_add(0x8));
        c.lwc1(0, node.wrapping_add(CRR_NODE_BOX + 0xc));
        c.swc1(0, s.wrapping_add(0x10));
        c.lwc1(1, node.wrapping_add(CRR_NODE_BOX + 0x10));
        c.swc1(1, s.wrapping_add(0x14));
        c.lwc1(0, node.wrapping_add(CRR_NODE_BOX + 0x14));
        c.swc1(20, s.wrapping_add(0x1c));
        c.swc1(0, s.wrapping_add(0x18));
        call_ptr(c, CRR_SPATIAL_KEY, 0x003a_b1a8)?;
        // Pick the hash (as `crr_insert`, with s5 = -1 and t0 for z).
        let s = sp(c);
        c.g[V0][0] = Addr(s.wrapping_add(0x24)).lw(c);
        c.g[V1][0] = sx(r32(c, S0).wrapping_add(0x50));
        c.g[A1][0] = Addr(s.wrapping_add(0x28)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[T0][0] = Addr(s.wrapping_add(0x2c)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[S0][0];
        }
        c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[A0][0] = sx(r32(c, V1).wrapping_add(0x28));
        c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
        if c.g[A1][0] != 0 {
            c.g[A0][0] = c.g[V1][0];
        }
        c.g[A3][0] = sx(s.wrapping_add(0x20));
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x14));
        c.g[A1][0] = 2;
        if c.g[T0][0] == 0 {
            c.g[A0][0] = c.g[V0][0];
        }
        call_ptr(c, CRR_SPATIAL_INSERT, 0x003a_b1e8)?;
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(CRR_OBJ_COUNT)).lw(c);
        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = Addr(r32(c, S1).wrapping_add(CRR_NODE_NEXT)).lw(c);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_b158);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 12 (0x3ab2dc): resolver virtual +0x98 `(rid, data)`.
pub(crate) fn crr_generic(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 9;
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(CRR_VTABLE)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(CRR_VT_GENERIC)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(CRR_VT_GENERIC + 4)).lw(c);
    crr_vcall(c, 0x003a_b2f8)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 13 (0x3ab300): `cWorldSound_OnSectionLoaded(gameModule->+0x4c,
/// data, rid)`.
pub(crate) fn crr_world_sound(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 8;
    call_ptr(c, CRR_AUDIO_GAME_MODULE, 0x003a_b308)?;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
    c.g[A2][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, CRR_WORLD_SOUND_LOADED, 0x003a_b318)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 17 (0x3ab320): `CameraTriggerMan_ReloadIf_Global(rid, data)`.
pub(crate) fn crr_camera_triggers(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, CRR_CAMERA_RELOAD, 0x003a_b32c)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 15 (0x3ab334): `cWorldPainterMan_SetFileData(res[2], rid & 0xff,
/// data)` when the script resources exist.
pub(crate) fn crr_painter(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(CRR_GP_SCRIPT_RES)).lw(c);
    c.g[A1][0] = c.g[S0][0] & 0xff;
    if c.g[V0][0] == 0 {
        return Ok(CrrExit::Plain);
    }
    c.cycles += 5;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x8)).lw(c);
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, CRR_PAINTER_SET, 0x003a_b34c)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 16 (0x3ab354): `cWScriptCache_SetData(res[1], rid & 0xff, data)`,
/// then `GameModule_CreateSectionScriptTable(SSXApp_GetGameModule(), rid &
/// 0xff)` when the script resources exist.
pub(crate) fn crr_script_table(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(CRR_GP_SCRIPT_RES)).lw(c);
    c.g[S0][0] &= 0xff;
    if c.g[V0][0] == 0 {
        return Ok(CrrExit::Plain);
    }
    c.cycles += 11;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x4)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S2][0];
    CRR_SCRIPT_CACHE_SET.call(c, 0x003a_b370)?;
    CRR_APP_GAME_MODULE.call(c, 0x003a_b378)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, CRR_SCRIPT_TABLE, 0x003a_b384)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 18 (0x3ab38c): `ScriptEngine_SetLocatorTable(*(gp-0x84c),
/// rid & 0xff, data)`.
pub(crate) fn crr_locators(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 6;
    c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(CRR_GP_LOCATORS_NEG)).lw(c);
    c.g[A1][0] = c.g[S0][0] & 0xff;
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, CRR_LOCATORS_SET, 0x003a_b39c)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 20 (0x3ab3a4): `SSXAudioSystem_LoadMemoryBank(GetInstance(),
/// rid, data)`.
pub(crate) fn crr_sound_bank(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 8;
    call_ptr(c, CRR_AUDIO_INSTANCE, 0x003a_b3ac)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, CRR_AUDIO_LOAD_BANK, 0x003a_b3bc)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 19 (0x3ab3c4): the mission interface (BE interface 10) gets
/// `(rid & 0xff, data + 4, *data)` (an empty stub in this build).
pub(crate) fn crr_mission(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 12;
    call_ptr(c, CRR_GET_BE, 0x003a_b3cc)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xa;
    call_ptr(c, CRR_BE_INTERFACE, 0x003a_b3d8)?;
    c.g[A3][0] = Addr(r32(c, S2)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0] & 0xff;
    c.g[A2][0] = sx(r32(c, S2).wrapping_add(0x4));
    call_ptr(c, CRR_MISSION_STUB, 0x003a_b3ec)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 11 (0x3ab404): `OcclusionMan_AddOccluder(GetOcclusionMan(), data)`.
pub(crate) fn crr_occluder(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 7;
    CRR_OCCLUSION_MAN.call(c, 0x003a_b40c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, CRR_ADD_OCCLUDER, 0x003a_b418)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

/// Message 14 (0x3ab420): `AI_SetTrackPathsBuffer(world->+0x84->+0xc,
/// rid & 0xff, rid >> 8, data)`; a rejected buffer is freed.
pub(crate) fn crr_track_paths(c: &mut Cpu) -> Result<CrrExit, Unwound> {
    c.cycles += 9;
    c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(CRR_GP_WORLD_NEG)).lw(c);
    c.g[A2][0] = sx(r32(c, S0) >> 8);
    c.g[A1][0] = c.g[S0][0] & 0xff;
    c.g[A3][0] = c.g[S2][0];
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
    call_ptr(c, CRR_TRACK_PATHS, 0x003a_b43c)?;
    let kept = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[S2][0];
    if kept {
        return Ok(CrrExit::Done);
    }
    c.cycles += 2;
    let no_data = c.g[S2][0] == 0;
    let s0_at = sp(c).wrapping_add(0xa0);
    c.lq(S0, s0_at);
    if no_data {
        return Ok(CrrExit::Epilogue);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, CRR_MEM_FREE, 0x003a_b454)?;
    c.g[V0][0] = c.g[S2][0];
    Ok(CrrExit::Done)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;

    use crate::difftest::fill;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::be::bedbreward::tests::alloc_ready as alloc_ready_2;

    pub(crate) use crate::game::bx::bxstringctor::tests::base_3;

    pub(crate) use crate::game::camera::cameracontroller::tests::heap_block;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::alloc_machine;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_7;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_6;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::f;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::put_vec3;
    pub(crate) use crate::game::render::particle::tests::uf;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    use ssx3_ee as ee;

    #[test]
    fn c_world_cache_streamer_get_section_matches() {
        check(
            ssx3_ee::s13::f_003a9820,
            c_world_cache_streamer_get_section,
            200,
            &|c, seed| {
                let mut r = Rng(0x9999 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u64;
                c.g[A1][0] = index;
                let count = r.next_u32() % 8;
                let head = if seed % 5 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA + 0x4, head);
                c.bus.write32(DATA + 0x8, DATA + 0x1000);
                c.bus.write32(head + 0x8, count);
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_is_section_resident_matches() {
        check(
            ssx3_ee::s13::f_003a9858,
            c_world_cache_streamer_is_section_resident,
            200,
            &|c, seed| {
                let mut r = Rng(0xaaaa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u32;
                c.g[A1][0] = index as u64;
                let count = r.next_u32() % 8;
                let head = if seed % 5 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA + 0x4, head);
                c.bus.write32(head + 0x8, count);
                c.bus.write32(
                    DATA + index * 8 + 0x14,
                    if seed % 2 == 0 { 3 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_is_section_unloaded_matches() {
        check(
            ssx3_ee::s13::f_003a9890,
            c_world_cache_streamer_is_section_unloaded,
            200,
            &|c, seed| {
                let mut r = Rng(0xbbbb ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (r.next_u32() % 8) as u32;
                c.g[A1][0] = index as u64;
                let count = r.next_u32() % 8;
                let head = if seed % 5 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA + 0x4, head);
                c.bus.write32(head + 0x8, count);
                c.bus.write32(
                    DATA + index * 8 + 0x14,
                    if seed % 2 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_get_hull_matches() {
        check_lohi(
            ssx3_ee::s13::f_003a9a98,
            c_world_cache_streamer_get_hull,
            200,
            &|c, seed| {
                let mut r = Rng(0xcccc ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 8) as u64;
                c.bus.write32(DATA + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_get_hull_count_matches() {
        check(
            ssx3_ee::s13::f_003a9ab0,
            c_world_cache_streamer_get_hull_count,
            200,
            &|c, seed| {
                let mut r = Rng(0xdddd ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let owner = DATA + 0x100;
                c.bus.write32(DATA + 0x4, owner);
                c.bus.write32(owner + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn world_cache_streamer_update_view_node_matches() {
        check(
            ee::s13::f_003aa028__generated,
            world_cache_streamer_update_view_node,
            SEEDS,
            &|c, seed| {
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
                    c.bus
                        .write32(rec + 0x20 + 4 * k, f(lo + uf(&mut r, 0.0, 16.0)));
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
                        c.bus
                            .write32(nd + 0x10 + 4 * k, f(lo + uf(&mut r, 0.0, 10.0)));
                    }
                    for k in 0..8 {
                        c.bus.write32(nd + 0x20 + 4 * k, f(uf(&mut r, -1.0, 1.0)));
                    }
                    let hull = hulls + 0x10 * i;
                    c.bus.write16(hull + 2, (r.next_u32() % 8) as u16);
                    c.bus
                        .write32(nd + 0x58, if r.next_u32() % 4 == 0 { 0 } else { hull });
                    let (left, right) = if i == 0 {
                        (DATA + 0x1100, DATA + 0x1200)
                    } else {
                        (0, 0)
                    };
                    c.bus
                        .write32(nd + 0x50, if seed % 3 == 0 { 0 } else { left });
                    c.bus
                        .write32(nd + 0x54, if seed % 5 == 0 { 0 } else { right });
                }
                c.g[A0][0] = sx(wc);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = view as u64;
                c.g[A3][0] = (seed / 2 % 2) as u64;
            },
        );
    }

    #[test]
    fn cw_script_cache_ctor_matches() {
        check(
            ee::s13::f_003ac7c8__generated,
            cw_script_cache_ctor,
            60,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x3ac7_c8);
                let this = DATA + 0x400;
                fill(c, &mut r, this, 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 3);
            },
        );
    }

    #[test]
    fn ps2_chunk_resolve_v14_matches() {
        check(
            ssx3_ee::s13::f_003aa5c8__generated,
            ps2_chunk_resolve_v14,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa5_c8, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                w(c, DATA + 0x200 + 8, (seed as u32) << 4);
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x78]);
                    w(c, DATA + 8, vt);
                }
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x3aa3f0 cPS2ChunkResolveBase_v05.

    #[test]
    fn c_ps2_chunk_resolve_base_v05_matches() {
        check(
            ee::s13::f_003aa3f0__generated,
            c_ps2_chunk_resolve_base_v05,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa3_f0, 1);
                let resolver = DATA + 0x100;
                let chunk = DATA + 0x200;
                c.g[A0][0] = sx(resolver);
                c.g[A1][0] = sx(chunk);
                c.bus.write32(resolver + 8, DATA + 0x300);
                c.bus.write32(chunk + 8, r.next_u32());
                c.bus.write32(chunk + 0xc, r.next_u32());
                let vt = vtable(c, &mut r, &[0x30]);
                c.bus.write32(DATA + 0x300, vt);
            },
        );
    }

    #[test]
    fn c_ps2_chunk_resolve_create_matches() {
        check(
            ee::s13::f_003aad98__generated,
            c_ps2_chunk_resolve_create,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x3aad_98, 0);
                alloc_ready(c);
            },
        );
    }

    #[test]
    fn c_ps2_chunk_resolve_dtor_matches() {
        check(
            ee::s13::f_003aade8__generated,
            c_ps2_chunk_resolve_dtor,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x3aade8, 2);
                alloc_ready_2(c);
                let this = heap_block(c, 0x100);
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    pub(crate) const SEEDS_5: u32 = 150;

    /// A float whose bits come from a small set, so every branch of a wrap or
    /// clamp function is reached.
    pub(crate) fn pick(r: &mut Rng, values: &[f32]) -> u32 {
        values[(r.next_u32() as usize) % values.len()].to_bits()
    }

    pub(crate) fn word(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    #[test]
    fn cps2_chunk_resolve_v16_matches() {
        check(
            ssx3_ee::s13::f_003aa6e8__generated,
            cps2_chunk_resolve_v16,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa6e8, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                word(c, DATA + 0x120, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v12_matches() {
        check(
            ssx3_ee::s13::f_003aa8d8__generated,
            cps2_chunk_resolve_v12,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa8d8, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = r.next_u32() as u64;
                word(c, DATA + 0x104, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_init_matches() {
        check(
            ssx3_ee::s13::f_003aae40__generated,
            cps2_chunk_resolve_init,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aae40, 0);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                word(
                    c,
                    crate::difftest::GAME_GP.wrapping_add(0x16c8),
                    DATA + 0x400,
                );
                word(c, DATA + 0x400, DATA + 0x500);
                word(c, DATA + 0x500, DATA + 0x600);
                c.bus
                    .write16(DATA + 0x600 + 0x2a, (r.next_u32() & 0xffff) as u16);
                word(c, DATA + 0xc, r.next_u32());
                word(c, DATA + 0x10, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------ chunk resolver

    /// The hash structures the resolver walks: `self -> *(self) -> +8` section
    /// table, then each section's three inner tables (`+0x4`, `+0x14`,
    /// `+0x24`). Entry 0 of every inner table is the "missing" case.
    pub(crate) fn resolver(c: &mut Cpu, self_: u32) {
        let p1 = self_ + 0x100;
        let p2 = self_ + 0x200;
        let sections = self_ + 0x300;
        let sec = self_ + 0x800;
        word(c, self_, p1);
        word(c, p1, p2);
        word(c, p2 + 8, sections);
        for i in 0..256 {
            word(c, sections + 4 * i, sec);
        }
        word(c, sec + 0xc, 0x1234);
        for (off, inner) in [
            (0x24u32, self_ + 0x900),
            (0x14, self_ + 0xa00),
            (0x4, self_ + 0xb00),
        ] {
            word(c, sec + off, inner);
            for i in 0..256u32 {
                let v = if i == 0 { 0 } else { (i << 8) | 0x40 };
                word(c, inner + 4 * i, v);
            }
        }
    }

    #[test]
    fn cps2_chunk_resolve_v17_matches() {
        check(
            ssx3_ee::s13::f_003aa700__generated,
            cps2_chunk_resolve_v17,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa700, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                resolver(c, DATA);
                let h = match seed % 4 {
                    0 => 0,
                    1 => 0x0000_0042,
                    2 => 0x0000_1042,
                    _ => r.next_u32() & 0xffff,
                };
                word(c, DATA + 0x1064, h);
                word(c, DATA + 0x1084, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_base_v13_matches() {
        check(
            ssx3_ee::s13::f_003aa8f0__generated,
            cps2_chunk_resolve_base_v13,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa8f0, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                resolver(c, DATA);
                let h = match seed % 4 {
                    0 => 0,
                    1 => 0x0000_0042,
                    2 => 0x0000_1042,
                    _ => r.next_u32() & 0xffff,
                };
                word(c, DATA + 0x1080, h);
                word(c, DATA + 0x108c, r.next_u32());
                word(c, DATA + 0x1094, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v09_matches() {
        check(
            ssx3_ee::s13::f_003aa758__generated,
            cps2_chunk_resolve_v09,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa758, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                word(
                    c,
                    DATA + 0x110,
                    match seed % 3 {
                        0 => 0xffff_ffff,
                        1 => 0,
                        _ => r.next_u32(),
                    },
                );
                word(c, DATA + 0x114, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v10_matches() {
        check(
            ssx3_ee::s13::f_003aa780__generated,
            cps2_chunk_resolve_v10,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa780, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 3) as u64; // 0, 1 or 2 counted arrays
                resolver(c, DATA);
                let data = DATA + 0x1000;
                c.g[A2][0] = sx(data);
                let mut at = data;
                let n = (seed % 3).max(1);
                for _ in 0..n {
                    let count = 1 + (r.next_u32() % 3);
                    word(c, at, count);
                    for j in 0..count {
                        let h = match r.next_u32() % 4 {
                            0 => 0,
                            1 => 0x42,
                            2 => 0x1042,
                            _ => r.next_u32() & 0xffff,
                        };
                        word(c, at + 4 + 4 * j, h);
                    }
                    at += 4 * (count + 1);
                }
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v18_matches() {
        check_lohi(
            ssx3_ee::s13::f_003aa960__generated,
            cps2_chunk_resolve_v18,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa960, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                let n = 1 + (seed % 3);
                word(c, DATA + 0x120, n);
                word(c, DATA + 0x124, r.next_u32());
                for i in 0..n {
                    let node = DATA + 0x200 + 0x90 * i;
                    let prev = match r.next_u32() % 4 {
                        0 => 0xffff_ffff,
                        1 => 0,
                        2 => n - 1,
                        _ => r.next_u32() % n,
                    };
                    let next = match r.next_u32() % 4 {
                        0 => 0xffff_ffff,
                        1 => 1 % n,
                        _ => r.next_u32() % n,
                    };
                    word(c, node + 0x60, prev);
                    word(c, node + 0x64, next);
                    word(c, node + 0x68, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v21_matches() {
        check(
            ssx3_ee::s13::f_003aac50__generated,
            cps2_chunk_resolve_v21,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aac50, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                word(
                    c,
                    crate::difftest::GAME_GP.wrapping_sub(0x23b8),
                    5000.0f32.to_bits(),
                );
                word(
                    c,
                    DATA + 0x10c,
                    if seed % 3 == 0 { 0x100 } else { r.next_u32() },
                );
                word(
                    c,
                    DATA + 0x110,
                    match seed % 5 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        _ => 0xffff_ffff,
                    },
                );
                word(
                    c,
                    DATA + 0x11c,
                    pick(&mut r, &[0.0, 4999.0, 5000.0, 5001.0, 1.0e9]),
                );
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v22_matches() {
        check(
            ssx3_ee::s13::f_003aaca8__generated,
            cps2_chunk_resolve_v22,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aaca8, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                word(
                    c,
                    DATA + 0x10c,
                    if seed % 3 == 0 { 0x100 } else { r.next_u32() },
                );
                word(
                    c,
                    DATA + 0x110,
                    match seed % 6 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        4 => 4,
                        _ => 0xffff_ffff,
                    },
                );
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v10_delivers_events() {
        use crate::difftest::events::schedule;
        check(
            ssx3_ee::s13::f_003aa780__generated,
            cps2_chunk_resolve_v10,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa780, 0);
                resolver(c, DATA);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 1;
                c.g[A2][0] = sx(DATA + 0x1000);
                let count = 6;
                word(c, DATA + 0x1000, count);
                for j in 0..count {
                    word(c, DATA + 0x1004 + 4 * j, r.next_u32() & 0xffff);
                }
                schedule(c, &mut r, seed, 0x003a_a780);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v18_delivers_events() {
        use crate::difftest::events::schedule;
        check_lohi(
            ssx3_ee::s13::f_003aa960__generated,
            cps2_chunk_resolve_v18,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3aa960, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                word(c, DATA + 0x120, 6);
                for i in 0..6 {
                    let node = DATA + 0x200 + 0x90 * i;
                    word(c, node + 0x60, if i == 0 { 0xffff_ffff } else { i - 1 });
                    word(c, node + 0x64, if i == 5 { 0xffff_ffff } else { i + 1 });
                }
                schedule(c, &mut r, seed, 0x003a_a960);
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_activate_section_matches() {
        check(
            ssx3_ee::s13::f_003a99d8__generated,
            c_world_cache_streamer_activate_section,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a99d8, 0);
                let cache = DATA + 0x1000;
                let idx = (seed % 4) as u32;
                c.g[A0][0] = cache as u64;
                c.g[A1][0] = idx as u64;
                let sections = if seed % 5 == 0 { 0 } else { DATA + 0x2000 };
                w32(c, cache + 4, sections);
                w32(c, sections + 8, if seed % 4 == 0 { 2 } else { 8 });
                for i in 0..8u32 {
                    w32(
                        c,
                        cache + 0x14 + i * 8,
                        if i == idx {
                            (seed % 8) as u32
                        } else {
                            r.next_u32() % 8
                        },
                    );
                }
                // The flagging path needs a world whose section has no objects.
                let world = DATA + 0x3000;
                let table = DATA + 0x4000;
                let lists = DATA + 0x5000;
                w32(c, cache, world);
                w32(c, world + 0x14, table);
                w16(c, table + idx * 0x58 + 0x22, 0);
                w32(c, world + 8, lists);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v20_matches() {
        check(
            ee::s13::f_003aabd8__generated,
            cps2_chunk_resolve_v20,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x3aabd8, 2);
                be_library(c);
                world(c);
                let chunk = DATA;
                c.g[A1][0] = chunk as u64;
                w(c, chunk + 0xc, (seed * 0x11) & 0x70);
                let mut r = Rng(0x5a5a_0000 ^ seed as u64);
                w(c, chunk + 0x1c, r.edge_f32());
                w(c, chunk + 0x20, r.edge_f32());
                w(c, chunk + 0x24, r.edge_f32());
                c.f[0] = r.edge_f32();
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v07_matches() {
        check(
            ee::s13::f_003aa520__generated,
            cps2_chunk_resolve_v07,
            8,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3aa520, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0x100;
                let vt7 = vtable(c, &mut r, &[0x40, 0x44, 0x60, 0x64]);
                w(c, OBJ + 8, vt7);
                let n = (seed % 3) as u32;
                w(c, OBJ2 + 0x1c, n);
                w(c, OBJ2 + 0x20, 0);
                w(c, OBJ2 + 0x24, 0);
                w(c, OBJ2 + 0x28, 0);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v11_matches() {
        check(
            ee::s13::f_003aa830__generated,
            cps2_chunk_resolve_v11,
            8,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3aa830, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = 0x100;
                let vt8 = vtable(c, &mut r, &[0x40, 0x44, 0x60, 0x64]);
                w(c, OBJ + 8, vt8);
                let n = (seed % 3) as u32;
                w(c, OBJ2 + 0x1c, n);
                w(c, OBJ2 + 0x20, 0);
                w(c, OBJ2 + 0x24, 0);
                w(c, OBJ2 + 0x28, 0);
            },
        );
    }

    #[test]
    fn cps2_chunk_resolve_v13_matches() {
        check(
            ee::s13::f_003ac2a8__generated,
            cps2_chunk_resolve_v13,
            80,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x3ac2);
                let instance = DATA + 0x300;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(instance);
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write32(DATA + 0x100, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 8, DATA + 0x400);
                let variant = seed % 4;
                let index = if variant == 0 { 0x55 } else { 0x34 };
                c.bus.write32(instance + 0x80, index);
                c.bus
                    .write32(DATA + 0x400 + (index & 0xff) * 4, DATA + 0x500);
                // The second-level value is a handle whose upper bits become a
                // small offset; keep the low RAM words plausible.
                c.bus.write32(0x20, 0x11);
                c.bus.write32(0x24, 0x22);
                fill(c, &mut r, instance + 0x8c, 0x10);
            },
        );
    }

    #[test]
    fn cps2chunkresolve_v05_matches() {
        check(
            ee::s13::f_003abe40__generated,
            cps2chunkresolve_v05,
            8,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x3abe40, 2);
                let this = DATA;
                let chunk = DATA + 0x200;
                clear_2(c, this, 0x400);
                clear_2(c, chunk, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = chunk as u64;
                w(c, chunk + 8, 0x10);
                w(c, chunk + 0xc, 0x20);
                w(c, chunk + 0x20, if seed % 2 == 0 { 0 } else { 0 });
                let vt = vtable(c, &mut r, &[0x30]);
                w(c, this + 8, vt);
            },
        );
    }

    #[test]
    fn cps2chunkresolve_relocate_instance_dma_matches() {
        check(
            ee::s13::f_003ac358__generated,
            cps2chunkresolve_relocate_instance_dma,
            8,
            &|c, seed| {
                let _ = base_4(c, seed, 0x3ac358, 2);
                let this = DATA;
                let chunk = DATA + 0x200;
                clear_2(c, this, 0x400);
                clear_2(c, chunk, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = chunk as u64;
                w(c, chunk + 0x98, 0x10);
                w(c, chunk + 0x80, DATA + 0x600);
                clear_2(c, DATA + 0x600, 0x100);
                w(
                    c,
                    DATA + 0x600 + 4,
                    if seed % 2 == 0 { 0 } else { 0xffff_ffff },
                );
                w(c, DATA + 0x600 + 8, 0);
            },
        );
    }

    #[test]
    fn c_ps2_chunk_resolve_v15_matches() {
        diff(
            ee::s13::f_003aa608__generated,
            c_ps2_chunk_resolve_v15,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x3a_a608);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                w(c, DATA + 0x2004, if seed % 3 == 0 { 0 } else { 1 });
                w(c, DATA + 0x2008, DATA + 0x3000);
                w(c, DATA + 0x3000, r.next_u32());
                w(c, DATA + 0x3004, if seed % 2 == 0 { 0 } else { 0x10 });
                w(c, DATA + 0x3008, r.next_u32());
                w(c, DATA + 0x300c, 0xffff_ffff);
                w(c, DATA + 8, 0); // no vtable on the chunk: calls are fakes
            },
        );
    }

    #[test]
    fn c_ps2_chunk_resolve_v06_matches() {
        diff(
            ee::s13::f_003aa438__generated,
            c_ps2_chunk_resolve_v06,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x3a_a438);
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(DATA);
                w(c, DATA + 0x2004, if seed % 3 == 0 { 0 } else { 1 });
                w(c, DATA + 0x2008, DATA + 0x3000);
                w(c, DATA + 0x3000, r.next_u32());
                w(c, DATA + 0x3004, if seed % 2 == 0 { 0 } else { 0x10 });
                w(c, DATA + 0x3008, r.next_u32());
                w(c, DATA + 8, 0);
            },
        );
    }

    /// cWorldCache_PointBoxDistance: per axis the point lies below, inside
    /// or above the box (seed enumerates all 27 combinations, so inside,
    /// one-axis and two/three-axis Euclidean paths are all reached), plus
    /// edge-case floats (±0, denormals, exponent 255, inverted boxes) on
    /// every fourth seed.
    #[test]
    fn world_cache_point_box_distance_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const POS: u32 = DATA;
        const BMIN: u32 = DATA + 0x10;
        const BMAX: u32 = DATA + 0x20;
        check(
            ee::s13::f_003a9e50__generated,
            world_cache_point_box_distance,
            300,
            &|c, seed| {
                let mut r = Rng(0x3a9e_5000 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x003a_9e50;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(POS);
                c.g[A2][0] = sx(BMIN);
                c.g[A3][0] = sx(BMAX);
                let mut mode = seed % 27;
                for axis in 0..3u32 {
                    let (p, lo, hi) = if seed % 4 == 3 {
                        (r.edge_f32(), r.edge_f32(), r.edge_f32())
                    } else {
                        let lo = (r.next_u32() % 200) as f32 - 100.0;
                        let hi = lo + (r.next_u32() % 50) as f32 + 0.25;
                        let p = match mode % 3 {
                            0 => lo - 1.0 - (r.next_u32() % 300) as f32 * 0.5,
                            1 => match r.next_u32() % 4 {
                                0 => lo,
                                1 => hi,
                                _ => (lo + hi) * 0.5,
                            },
                            _ => hi + 1.0 + (r.next_u32() % 300) as f32 * 0.5,
                        };
                        (p.to_bits(), lo.to_bits(), hi.to_bits())
                    };
                    mode /= 3;
                    c.bus.write32(POS + axis * 4, p);
                    c.bus.write32(BMIN + axis * 4, lo);
                    c.bus.write32(BMAX + axis * 4, hi);
                }
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_words;
    use crate::difftest::GAME_GP;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's callee fakes (in the machine's RAM).
    pub(crate) const CCR_CTRL: u32 = DATA + 0xf_fff0;

    pub(crate) const CCR_MAGIC: u32 = 0x5eed_b498;

    /// The message jump table (.rodata 0x495010) from the game image.
    pub(crate) const CCR_TABLE_WORDS: [u32; 23] = [
        0x003a_be10,
        0x003a_b4f0,
        0x003a_be10,
        0x003a_b664,
        0x003a_be10,
        0x003a_bbb0,
        0x003a_b918,
        0x003a_ba70,
        0x003a_b798,
        0x003a_b628,
        0x003a_b638,
        0x003a_bdcc,
        0x003a_be10,
        0x003a_bd04,
        0x003a_bde8,
        0x003a_bd34,
        0x003a_be10,
        0x003a_bd24,
        0x003a_bd60,
        0x003a_bd94,
        0x003a_bd74,
        0x003a_bdbc,
        0x003a_be08,
    ];

    // Fixture layout.
    pub(crate) const CCR_SELF: u32 = DATA + 0x4_0000;

    pub(crate) const CCR_VT6: u32 = DATA + 0x4_1000;

    pub(crate) const CCR_HASH: u32 = DATA + 0x4_2000;

    pub(crate) const CCR_NODES: u32 = DATA + 0x4_3000;

    pub(crate) const CCR_ITEM: u32 = DATA + 0x5_0000;

    pub(crate) const CCR_LIST: u32 = DATA + 0x5_1000;

    pub(crate) const CCR_MISSION: u32 = DATA + 0x6_0000;

    pub(crate) const CCR_MISSION_VT: u32 = DATA + 0x6_1000;

    pub(crate) const CCR_WPF: u32 = DATA + 0x6_2000;

    pub(crate) const CCR_WPF_TABLE: u32 = DATA + 0x6_3000;

    pub(crate) const CCR_WORLD: u32 = DATA + 0x6_5000;

    pub(crate) const CCR_AUDIO: u32 = DATA + 0x8_0000;

    macro_rules! ccr_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(CCR_CTRL) == CCR_MAGIC {
                    return fakes::fake_call(c, $addr);
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    ccr_gated!(
        ccr_fake_key,
        CCR_PREV_KEY,
        0x0032_8f28,
        ee::s10::f_00328f28__generated
    );

    ccr_gated!(
        ccr_fake_remove,
        CCR_PREV_REMOVE,
        0x0032_84b8,
        ee::s10::f_003284b8__generated
    );

    ccr_gated!(
        ccr_fake_delete,
        CCR_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );

    ccr_gated!(
        ccr_fake_28b1c8,
        CCR_PREV_28B1C8,
        0x0028_b1c8,
        ee::s07::f_0028b1c8__generated
    );

    ccr_gated!(
        ccr_fake_278a58,
        CCR_PREV_278A58,
        0x0027_8a58,
        ee::s07::f_00278a58__generated
    );

    ccr_gated!(
        ccr_fake_28b180,
        CCR_PREV_28B180,
        0x0028_b180,
        ee::s07::f_0028b180__generated
    );

    ccr_gated!(
        ccr_fake_2b6b30,
        CCR_PREV_2B6B30,
        0x002b_6b30,
        ee::s08::f_002b6b30__generated
    );

    ccr_gated!(
        ccr_fake_16cf18,
        CCR_PREV_16CF18,
        0x0016_cf18,
        ee::s02::f_0016cf18__generated
    );

    ccr_gated!(
        ccr_fake_14dc80,
        CCR_PREV_14DC80,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );

    ccr_gated!(
        ccr_fake_14dd58,
        CCR_PREV_14DD58,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );

    ccr_gated!(
        ccr_fake_210608,
        CCR_PREV_210608,
        0x0021_0608,
        ee::s05::f_00210608__generated
    );

    ccr_gated!(
        ccr_fake_3ad198,
        CCR_PREV_3AD198,
        0x003a_d198,
        ee::s13::f_003ad198__generated
    );

    ccr_gated!(
        ccr_fake_242570,
        CCR_PREV_242570,
        0x0024_2570,
        ee::s06::f_00242570__generated
    );

    ccr_gated!(
        ccr_fake_12a490,
        CCR_PREV_12A490,
        0x0012_a490,
        ee::s00::f_0012a490__generated
    );

    ccr_gated!(
        ccr_fake_2d9b40,
        CCR_PREV_2D9B40,
        0x002d_9b40,
        ee::s09::f_002d9b40__generated
    );

    /// Gated fakes in the callees' override slots (once per process). The
    /// callees without a slot run for real on short paths: WPFile_Unload on
    /// an unloaded entry, the bank unload on a slot that does not match, the
    /// empty BE stub.
    #[allow(unused_unsafe)]
    pub(crate) fn ccr_install_fakes() {
        use std::ptr::addr_of_mut;
        unsafe {
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00328F28),
                ccr_fake_key,
                &CCR_PREV_KEY,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_003284B8),
                ccr_fake_remove,
                &CCR_PREV_REMOVE,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00317E50),
                ccr_fake_delete,
                &CCR_PREV_DELETE,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_0028B1C8),
                ccr_fake_28b1c8,
                &CCR_PREV_28B1C8,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00278A58),
                ccr_fake_278a58,
                &CCR_PREV_278A58,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_0028B180),
                ccr_fake_28b180,
                &CCR_PREV_28B180,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002B6B30),
                ccr_fake_2b6b30,
                &CCR_PREV_2B6B30,
            );
            fakes::hook(
                addr_of_mut!(ee::s02::OVERRIDE_0016CF18),
                ccr_fake_16cf18,
                &CCR_PREV_16CF18,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
                ccr_fake_14dc80,
                &CCR_PREV_14DC80,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
                ccr_fake_14dd58,
                &CCR_PREV_14DD58,
            );
            fakes::hook(
                addr_of_mut!(ee::s05::OVERRIDE_00210608),
                ccr_fake_210608,
                &CCR_PREV_210608,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003AD198),
                ccr_fake_3ad198,
                &CCR_PREV_3AD198,
            );
            fakes::hook(
                addr_of_mut!(ee::s06::OVERRIDE_00242570),
                ccr_fake_242570,
                &CCR_PREV_242570,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0012A490),
                ccr_fake_12a490,
                &CCR_PREV_12A490,
            );
            fakes::hook(
                addr_of_mut!(ee::s09::OVERRIDE_002D9B40),
                ccr_fake_2d9b40,
                &CCR_PREV_2D9B40,
            );
        }
    }

    /// One message per seed (half of them the spatial-hash ones, a few out
    /// of range, one with the upper half of a1 set), the hash's eight root
    /// nodes with zero or one non-empty word (so every scan exit is taken),
    /// random key signs, a 0..3-item list for message 8.
    pub(crate) fn ccr_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xb498, 6);
        c.bus.write32(CCR_CTRL, CCR_MAGIC);
        put_words(c, ccr_release::TABLE, &CCR_TABLE_WORDS);

        let msg: u64 = if seed % 2 == 0 {
            [1, 3, 5, 6, 7, 8][(seed / 2 % 6) as usize]
        } else {
            match seed / 2 % 25 {
                23 => 0x17 + (r.next_u32() % 0x100) as u64,
                24 => 1u64 << 32 | 1,
                m => m as u64,
            }
        };
        let param: u64 = if msg == 20 {
            // A bank slot that does not match: the unload returns at once.
            ((r.next_u32() % 16) << 8 | (1 + r.next_u32() % 255)) as u64
        } else {
            (r.next_u32() as u64) << 32 | r.next_u32() as u64
        };
        if msg == 20 {
            c.bus.write32(LOG_RESULTS, CCR_AUDIO);
        }

        // self: vtable (slot 22 at +0xb0), the hash, a base index.
        c.bus.write32(CCR_SELF + ccr_release::SELF_VT, CCR_VT6);
        c.bus.write32(CCR_SELF + ccr_release::SELF_HASH, CCR_HASH);
        c.bus
            .write32(CCR_SELF + ccr_release::SELF_BASE, r.next_u32());
        c.bus.write16(CCR_VT6 + 0xb0, (r.next_u32() as u16) & !3);
        let slot22 = if r.next_u32() % 3 == 0 {
            0x0014_06c0
        } else {
            FAKE
        };
        c.bus.write32(CCR_VT6 + 0xb4, slot22);

        // The hash: update counter, eight roots and their nodes.
        c.bus
            .write32(CCR_HASH + ccr_release::HASH_UPDATES, r.next_u32());
        for k in 0..8u32 {
            let node = CCR_NODES + 0x40 * k;
            c.bus
                .write32(CCR_HASH + 0x14 * k + ccr_release::ROOT_NODE, node);
            let busy = r.next_u32() % 14;
            for w in 0..11u32 {
                let v = if w == busy { r.next_u32() | 1 } else { 0 };
                c.bus.write32(node + 4 * w, v);
            }
        }
        // The key (x, y, z words) the fake KEY_FROM_BOX leaves on the stack.
        let key = crate::difftest::SP - 0xc0 + 0x24;
        for i in 0..3 {
            let v = r.next_u32();
            c.bus.write32(
                key + 4 * i,
                if r.next_u32() % 2 == 0 {
                    v >> 1
                } else {
                    v | 1 << 31
                },
            );
        }

        // data: boxes everywhere, a list of 0..3 items (count may be <= 0).
        for i in 0..0x60u32 {
            let f = r.f32(100.0);
            c.bus.write32(CCR_ITEM + 4 * i, f);
        }
        let count = (r.next_u32() % 5) as i32 - 1;
        c.bus
            .write32(CCR_ITEM + ccr_release::LIST_COUNT, count as u32);
        c.bus.write32(CCR_ITEM + ccr_release::LIST_FIRST, CCR_LIST);
        for i in 0..4u32 {
            let e = CCR_LIST + 0x100 * i;
            for w in 0..0x22u32 {
                let f = r.f32(100.0);
                c.bus.write32(e + 4 * w, f);
            }
            c.bus.write32(e + ccr_release::ITEM_NEXT, e + 0x100);
        }

        // gp globals: mission manager (vt slot 50 at +0x190), waypoint files
        // (unloaded entries), an ambient pointer, the world.
        let gp = GAME_GP;
        c.bus.write32(gp + ccr_release::GP_MISSION_MAN, CCR_MISSION);
        c.bus.write32(CCR_MISSION + 0x10d8, CCR_MISSION_VT);
        c.bus
            .write16(CCR_MISSION_VT + 0x190, (r.next_u32() as u16) & !3);
        c.bus.write32(CCR_MISSION_VT + 0x194, FAKE + 0x40);
        let wpf = if r.next_u32() % 3 == 0 { 0 } else { CCR_WPF };
        c.bus.write32(gp + ccr_release::GP_WP_FILES, wpf);
        c.bus.write32(CCR_WPF + 0x8, CCR_WPF + 0x100);
        c.bus.write32(CCR_WPF + 0x104, CCR_WPF_TABLE);
        c.bus.write32(gp - ccr_release::GP_AMBIENT, r.next_u32());
        c.bus.write32(gp - ccr_release::GP_WORLD, CCR_WORLD);
        c.bus.write32(CCR_WORLD + 0x84, CCR_WORLD + 0x100);
        fill(c, &mut r, CCR_WORLD + 0x100, 8);

        c.g[A0][0] = CCR_SELF as u64;
        c.g[A1][0] = msg;
        c.g[A2][0] = param;
        c.g[A3][0] = CCR_ITEM as u64;
        c.pc = ccr_release::ENTRY;
    }

    /// The world view's message handler against the generated body.
    #[test]
    fn cps2_chunk_resolve_release_matches() {
        ccr_install_fakes();
        covered(
            ee::s13::f_003ab498__generated,
            cps2_chunk_resolve_release,
            500,
            &|c, seed| ccr_setup(c, seed),
        );
    }

    /// The gate of this module's callee fakes (outside the fakes' pointer
    /// range 0x800000..0x840000 and their log).
    pub(crate) const OC368_CTRL: u32 = 0x008f_0000;

    pub(crate) const OC368_MAGIC: u32 = 0x0c36_8a5e;

    /// The jump table's words (`.rodata` 0x494fb0, from the ELF).
    pub(crate) const OC368_CRR_TABLE: [u32; 23] = [
        0x003a_b080,
        0x003a_aec0,
        0x003a_b06c,
        0x003a_afc8,
        0x003a_b2bc,
        0x003a_b200,
        0x003a_b094,
        0x003a_b0dc,
        0x003a_b120,
        0x003a_af5c,
        0x003a_af88,
        0x003a_b404,
        0x003a_b2dc,
        0x003a_b300,
        0x003a_b420,
        0x003a_b334,
        0x003a_b354,
        0x003a_b320,
        0x003a_b38c,
        0x003a_b3c4,
        0x003a_b3a4,
        0x003a_b3f4,
        0x003a_b45c,
    ];

    /// Gated logging fakes in the callees' override slots: with the gate set
    /// a call is logged (`fakes::fake`: random pointer result, forced
    /// results, unwinds); otherwise the slot's previous body runs.
    macro_rules! oc368_gated_fakes {
        ($($name:ident: $slot:expr => $gen:expr, $addr:expr;)*) => {
            $(
                mod $name {
                    use super::{OC368_CTRL, OC368_MAGIC};
                    use crate::difftest::fakes;
                    use ssx3_ee as ee;
                    use ssx3_ee_rt::cpu::reg::RA;
                    use ssx3_ee_rt::Cpu;
                    use std::sync::atomic::AtomicUsize;
                    pub static PREV: AtomicUsize = AtomicUsize::new(0);
                    pub fn fake(c: &mut Cpu) {
                        if c.bus.read32(OC368_CTRL) != OC368_MAGIC {
                            return fakes::fall_back(c, &PREV, $gen);
                        }
                        let ra = c.g[RA][0] as u32;
                        fakes::fake(c, $addr, ra);
                    }
                }
            )*
            fn oc368_install_fakes() {
                $(
                    fakes::hook(std::ptr::addr_of_mut!($slot), $name::fake, &$name::PREV);
                )*
            }
        };
    }

    oc368_gated_fakes! {
        oc368_fk_spatial_key: ee::s10::OVERRIDE_00328F28 => ee::s10::f_00328f28__generated, 0x0032_8f28;
        oc368_fk_insert_at_key: ee::s10::OVERRIDE_00328660 => ee::s10::f_00328660__generated, 0x0032_8660;
        oc368_fk_alloc: ee::s10::OVERRIDE_00317D70 => ee::s10::f_00317d70__generated, 0x0031_7d70;
        oc368_fk_free: ee::s10::OVERRIDE_00317E98 => ee::s10::f_00317e98__generated, 0x0031_7e98;
        oc368_fk_memset: ee::s14::OVERRIDE_003E6448 => ee::s14::f_003e6448__generated, 0x003e_6448;
        oc368_fk_audio_module: ee::s07::OVERRIDE_0028B1C8 => ee::s07::f_0028b1c8__generated, 0x0028_b1c8;
        oc368_fk_locators: ee::s07::OVERRIDE_00278A10 => ee::s07::f_00278a10__generated, 0x0027_8a10;
        oc368_fk_audio_instance: ee::s07::OVERRIDE_0028B180 => ee::s07::f_0028b180__generated, 0x0028_b180;
        oc368_fk_load_bank: ee::s07::OVERRIDE_00286CA8 => ee::s07::f_00286ca8__generated, 0x0028_6ca8;
        oc368_fk_path_free: ee::s07::OVERRIDE_0026AF00 => ee::s07::f_0026af00__generated, 0x0026_af00;
        oc368_fk_path_set: ee::s07::OVERRIDE_0026AD70 => ee::s07::f_0026ad70__generated, 0x0026_ad70;
        oc368_fk_world_sound: ee::s08::OVERRIDE_002B6B08 => ee::s08::f_002b6b08__generated, 0x002b_6b08;
        oc368_fk_script_cache: ee::s13::OVERRIDE_003ACA10 => ee::s13::f_003aca10__generated, 0x003a_ca10;
        oc368_fk_app_module: ee::s13::OVERRIDE_003AD188 => ee::s13::f_003ad188__generated, 0x003a_d188;
        oc368_fk_occlusion_man: ee::s13::OVERRIDE_003AD198 => ee::s13::f_003ad198__generated, 0x003a_d198;
        oc368_fk_get_be: ee::s01::OVERRIDE_0014DC80 => ee::s01::f_0014dc80__generated, 0x0014_dc80;
        oc368_fk_be_interface: ee::s01::OVERRIDE_0014DD58 => ee::s01::f_0014dd58__generated, 0x0014_dd58;
        oc368_fk_race_path: ee::s05::OVERRIDE_002105D0 => ee::s05::f_002105d0__generated, 0x0021_05d0;
        oc368_fk_find_section: ee::s05::OVERRIDE_0022E0E0 => ee::s05::f_0022e0e0__generated, 0x0022_e0e0;
        oc368_fk_add_occluder: ee::s06::OVERRIDE_00242540 => ee::s06::f_00242540__generated, 0x0024_2540;
        oc368_fk_avalanche: ee::s09::OVERRIDE_002D9FB8 => ee::s09::f_002d9fb8__generated, 0x002d_9fb8;
    }

    /// Every message (0-22) and out-of-range ones, against fakes for the
    /// override-slot callees; the slotless callees run for real on short
    /// paths: SpatialHash_Insert (empty hash, then the same key again),
    /// WPFile_SetData (data not a relocatable file), the section script
    /// table (no scripts), the camera triggers (no data), the mission stub,
    /// AI_SetTrackPathsBuffer (forced section numbers 0x11..0x15 or a
    /// pointer; both results, freeing or not).
    #[test]
    fn c_ps2_chunk_resolve_resolve_matches() {
        oc368_install_fakes();
        fakes::covered(
            ee::s13::f_003aae68__generated,
            c_ps2_chunk_resolve_resolve,
            300,
            &|c, seed| {
                const BASE: u32 = 0x00a0_0000;
                const SELF: u32 = BASE;
                const VT: u32 = BASE + 0x100;
                const HASHES: u32 = BASE + 0x400;
                const OBJ: u32 = BASE + 0x1000;
                const NODES: u32 = BASE + 0x2000;
                const VIEW: u32 = BASE + 0x3000;
                const VIEW_VT: u32 = BASE + 0x3200;
                const RES: u32 = BASE + 0x4000;
                const RES_SCRIPTS: u32 = BASE + 0x4100;
                const RES_TABLE: u32 = BASE + 0x4200;
                const PAINTER: u32 = BASE + 0x5000;
                const PAINTER_FILES: u32 = BASE + 0x5100;
                const WORLD: u32 = BASE + 0x6000;
                const WORLD_AI: u32 = BASE + 0x6100;

                let mut r = fakes::machine(c, seed, 0x3aae68, 6);
                c.pc = CRR_ENTRY;
                c.bus.write32(OC368_CTRL, OC368_MAGIC);
                put_words(c, CRR_JUMP_TABLE, &OC368_CRR_TABLE);
                let msg = seed % 25;
                let round = seed / 25;
                c.g[A1][0] = match msg {
                    23 => 23 + (r.next_u32() & 0xff) as u64,
                    24 => sx(r.next_u32() | 0x8000_0000),
                    m => m as u64,
                };

                // The resolver and its vtable.
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + CRR_VTABLE, VT);
                c.bus.write32(SELF + CRR_HASHES, HASHES);
                c.bus.write32(SELF + CRR_BASE_INDEX, r.next_u32());
                let slots = [0x28, 0x48, 0x68, 0x70, 0x88, 0x90, 0x98, 0xa0, 0xa8];
                for (i, off) in slots.into_iter().enumerate() {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u16;
                    c.bus.write16(VT + off, delta);
                    c.bus.write32(VT + off + 4, FAKE + 0x10 * i as u32);
                }
                if round % 2 == 0 {
                    // Visibility test rejects (a real leaf returning 0).
                    c.bus.write32(VT + CRR_VT_VISIBLE + 4, 0x0014_06c0);
                }

                // The chunk: random boxes, a patch list.
                fill(c, &mut r, OBJ, 0x70);
                c.bus.write32(OBJ, 0x20);
                let count = [0i32, 1, 2, 3, -2][(round % 5) as usize];
                c.bus.write32(OBJ + CRR_OBJ_COUNT, count as u32);
                c.bus.write32(OBJ + CRR_OBJ_LIST, NODES);
                for i in 0..3 {
                    let node = NODES + 0x100 * i;
                    fill(c, &mut r, node + CRR_NODE_BOX, 6);
                    c.bus.write32(node + CRR_NODE_NEXT, node + 0x100);
                }
                let obj = match msg {
                    17 => 0,
                    14 if round % 3 == 0 => 0,
                    _ => OBJ,
                };
                c.g[A3][0] = sx(obj);
                let hi = [0u32, 1, 2, 3, 5, r.next_u32() >> 8][(round % 6) as usize];
                c.g[A2][0] = sx((hi << 8) | (r.next_u32() & 0xff));

                // The world view's stream handler.
                c.bus.write32(GAME_GP + CRR_GP_WORLD_VIEW, VIEW);
                c.bus.write32(VIEW + 0x10d8, VIEW_VT);
                c.bus
                    .write16(VIEW_VT + 0x170, (r.next_u32() & 0xfffc) as u16);
                c.bus.write32(VIEW_VT + 0x174, FAKE + 0x200);

                // Script resources (or none), painter files, the world.
                let res = if round % 4 == 1 { 0 } else { RES };
                c.bus.write32(GAME_GP + CRR_GP_SCRIPT_RES, res);
                c.bus.write32(RES + 4, RES_SCRIPTS);
                c.bus.write32(RES_SCRIPTS + 4, RES_TABLE);
                c.bus.write32(RES + 8, PAINTER);
                c.bus.write32(PAINTER + 4, PAINTER_FILES);
                c.bus.write32(GAME_GP - CRR_GP_LOCATORS_NEG, r.next_u32());
                c.bus.write32(GAME_GP - CRR_GP_WORLD_NEG, WORLD);
                c.bus.write32(WORLD + 0x84, WORLD_AI);
                c.bus.write32(WORLD_AI + 0x78, r.next_u32());
                if msg == 14 && round % 2 == 1 {
                    // AI_SetTrackPathsBuffer's section number (first call).
                    c.bus.write32(LOG_RESULTS, 0x11 + r.next_u32() % 5);
                }
            },
        );
    }
}
