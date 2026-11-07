//! `src/intersect/aifwddiff.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_reg;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw;
pub(crate) use crate::game::intersect::worldsphtree::fpu;
pub(crate) use crate::game::intersect::worldsphtree::lw;
pub(crate) use crate::game::intersect::worldsphtree::p;
pub(crate) use crate::game::intersect::worldsphtree::sw_reg;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::render::particle::add32;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::render::particle::ldf;
pub(crate) use crate::game::render::particle::lt;
pub(crate) use crate::game::render::particle::setf;
pub(crate) use crate::game::render::particle::shl32;
pub(crate) use crate::game::render::particle::sra32;
pub(crate) use crate::game::render::particle::srl32;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::float;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_7ac0, ai_fwd_diff_cache_touch, "AIFwdDiffCache_Touch"),
    (
        0x0032_7b30,
        ai_fwd_diff_cache_hash_remove,
        "AIFwdDiffCache_HashRemove",
    ),
    (
        0x0032_7bb0,
        ai_fwd_diff_cache_hash_insert,
        "AIFwdDiffCache_HashInsert",
    ),
    (
        0x0032_7cc8,
        roller_modifier_copy_matrix_block,
        "RollerModifier_CopyMatrixBlock",
    ),
    (
        0x0032_8030,
        sphere_tree_collide_triangle2_sided,
        "SphereTree_CollideTriangle2Sided",
    ),
    (
        0x0032_8360,
        spatial_key_classify_aabb,
        "SpatialKey_ClassifyAABB",
    ),
    (0x0032_8f28, spatial_key_from_aabb, "SpatialKey_FromAABB"),
    (0x0032_91e0, spatial_hash_update, "SpatialHash_Update"),
    (0x0032_79d0, ai_fwd_diff_cache_get, "AIFwdDiffCache_Get"),
    (
        0x0032_9590,
        collide_sphere_vs_box_edges,
        "Collide_SphereVsBoxEdges",
    ),
    (0x0032_8808, spatial_hash_move, "SpatialHash_Move"),
    (
        0x0032_8660,
        spatial_hash_insert_at_key,
        "SpatialHash_InsertAtKey",
    ),
    (
        0x0032_7c00,
        ai_fwd_diff_cache_build_entry,
        "AIFwdDiffCache_BuildEntry",
    ),
    (
        0x0032_7c68,
        ai_fwd_diff_cache_reset_entries,
        "AIFwdDiffCache_ResetEntries",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::render::particle::callee::ADD_DIRECTIONAL;
    pub(crate) use crate::game::render::particle::callee::BLEND_VIEW_COLOURS;
    pub(crate) use crate::game::render::particle::callee::COMPUTE_COLOUR_FACTOR;
    pub(crate) use crate::game::render::particle::callee::DELETE;
    pub(crate) use crate::game::render::particle::callee::DYNAMIC_EMITTER_SPAWN;
    pub(crate) use crate::game::render::particle::callee::FONT_DRAW_TEXT;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE0;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE1;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE2;
    pub(crate) use crate::game::render::particle::callee::GET_LIGHTING28;
    pub(crate) use crate::game::render::particle::callee::HUD_ALIGN_ELEM_POS;
    pub(crate) use crate::game::render::particle::callee::HUD_DRAW_PANEL_SEGMENT;
    pub(crate) use crate::game::render::particle::callee::HUD_DRAW_SPRITE;
    pub(crate) use crate::game::render::particle::callee::HUD_GET_ELEM_COLOR;
    pub(crate) use crate::game::render::particle::callee::HUD_GET_ELEM_POS;
    pub(crate) use crate::game::render::particle::callee::HUD_GET_ELEM_SIZE;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_ADD;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_SCALE;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_SCALE_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::IS_HULL_RESIDENT;
    pub(crate) use crate::game::render::particle::callee::LERP_VIEW_COLOURS;
    pub(crate) use crate::game::render::particle::callee::POINT_BOX_DISTANCE;
    pub(crate) use crate::game::render::particle::callee::POINT_LIGHT_VECTOR;
    pub(crate) use crate::game::render::particle::callee::RANDOM;
    pub(crate) use crate::game::render::particle::callee::REQUEST_HULL;
    pub(crate) use crate::game::render::particle::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::render::particle::callee::SAMPLE_SURFACE_COLOURS;
    pub(crate) use crate::game::render::particle::callee::SECTION_GET_EVENT_WORLD_SECTION;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_HASH_INSERT;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_HASH_MOVE;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_HASH_REMOVE;
    pub(crate) use crate::game::render::particle::callee::SPATIAL_KEY_FROM_AABB;
    pub(crate) use crate::game::render::particle::callee::SPOT_LIGHT_VECTOR;
    pub(crate) use crate::game::render::particle::callee::STREAMER_UPDATE_VIEW_NODE;
}

// ---------------------------------------------------------------------------
// Forward-difference cache (continued).

/// `void AIFwdDiffCache_Touch(AIFwdDiffCache *cache, void *node)`
/// (0x327ac0): move `node` to the front of the bucket it currently sits in,
/// or unlink it first when it is not the bucket head.
pub fn ai_fwd_diff_cache_touch(c: &mut Cpu) {
    c.cycles += 4;
    let cache = abi::arg(c, 0);
    let node = abi::arg(c, 1);
    c.g[A2][0] = cache as u64;
    let cur = Addr(cache).off(4).lw(c);
    c.g[V0][0] = cur;
    if cur != node as u64 {
        let next = Addr(cur as u32).off(8).lw(c);
        c.g[V0][0] = next;
        c.cycles += 2;
        if next != node as u64 {
            c.cycles += 16;
            let a = Addr(node).off(8).lw(c);
            c.g[V1][0] = a;
            let b = Addr(node).off(4).lw(c);
            c.g[V0][0] = b;
            Addr(a as u32).off(4).sw(c, b);
            let c1 = Addr(node).off(8).lw(c);
            c.g[V0][0] = c1;
            let d = Addr(node).off(4).lw(c);
            c.g[A0][0] = d;
            Addr(d as u32).off(8).sw(c, c1);
            let e = Addr(cache).off(4).lw(c);
            c.g[V1][0] = e;
            let f = Addr(e as u32).off(8).lw(c);
            c.g[V0][0] = f;
            Addr(node).off(8).sw(c, f);
            let g = Addr(cache).off(4).lw(c);
            c.g[V1][0] = g;
            Addr(node).off(4).sw(c, g);
            let h = Addr(cache).off(4).lw(c);
            c.g[V0][0] = h;
            let i = Addr(h as u32).off(8).lw(c);
            c.g[V1][0] = i;
            Addr(i as u32).off(4).sw(c, node as u64);
            let j = Addr(cache).off(4).lw(c);
            c.g[V0][0] = j;
            Addr(j as u32).off(8).sw(c, node as u64);
        }
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 3;
        let v = Addr(node).off(4).lw(c);
        c.g[V0][0] = v;
        Addr(cache).off(4).sw(c, v);
        abi::ret(c);
    }
}

/// `void AIFwdDiffCache_HashInsert(AIFwdDiffCache *this, Entry *e)`
/// (0x327bb0): insert `e` into the hash bucket `e->key % this->bucket_count`
/// (`this+0xc`), which is a list ordered by descending key (`e->key` at +0,
/// the chain link at +0xc).
pub fn ai_fwd_diff_cache_hash_insert(c: &mut Cpu) {
    const BUCKET_COUNT: u32 = 0xc;
    const BUCKETS: u32 = 0x8;
    const KEY: u32 = 0x0;
    const NEXT: u32 = 0xc;
    const HEAD: u32 = 0x0032_7bd8;
    c.cycles += 5;
    let this = c.g[A0][0] as u32;
    c.g[A3][0] = c.g[A1][0];
    c.g[V0][0] = Addr(this).off(BUCKET_COUNT).lw(c);
    c.g[A2][0] = Addr(c.g[A3][0] as u32).off(KEY).lw(c);
    c.cycles += 5;
    abi::divu(c, c.g[A2][0] as u32, c.g[V0][0] as u32);
    c.g[A1][0] = Addr(this).off(BUCKETS).lw(c);
    c.g[V1][0] = sx((c.hi[0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    loop {
        c.cycles += 3;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        if c.g[V1][0] == 0 {
            c.sw(c.g[A1][0] as u32, c.g[A3][0]);
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(KEY).lw(c);
        c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.sw(c.g[A1][0] as u32, c.g[A3][0]);
            break;
        }
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(NEXT));
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    let ret = c.g[RA][0] as u32;
    c.sw((c.g[A3][0] as u32).wrapping_add(NEXT), c.g[V1][0]);
    c.pc = ret;
}

// ---------------------------------------------------------------------------
// Assorted leaves and leaf loops.

/// `void AIFwdDiffCache_HashRemove(AIFwdDiffCache *this, Entry *e)`
/// (0x327b30): remove the hash entry whose key (`e->key`) is `e` from the
/// bucket `key % this->bucket_count`. A `match pc` dispatcher re-enterable at
/// 0x327b78.
pub fn ai_fwd_diff_cache_hash_remove(c: &mut Cpu) {
    const ENTRY: u32 = 0x0032_7b30;
    const END: u32 = 0x0032_7bb0;
    const RESUME: u32 = 0x0032_7b78;
    let mut stage = match c.pc {
        ENTRY => 0u8,
        RESUME => 4,
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    loop {
        match stage {
            0 => {
                c.cycles += 5;
                c.g[V0][0] = sx(0xffff_0000) | 0xffff;
                c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
                stage = if c.g[A1][0] == c.g[V0][0] { 8 } else { 1 };
            }
            1 => {
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
                abi::divu(c, c.g[A1][0] as u32, c.g[V0][0] as u32);
                stage = 2;
            }
            2 => {
                c.cycles += 7;
                c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                c.g[V1][0] = sx((c.hi[0] as u32) << 2);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V1][0] = c.g[V0][0];
                stage = if c.g[V0][0] == 0 { 8 } else { 3 };
            }
            3 => {
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                stage = 4;
            }
            4 => {
                c.cycles += 2;
                if c.g[V0][0] == c.g[A1][0] {
                    stage = 5;
                } else {
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                    stage = 6;
                }
            }
            5 => {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                stage = 8;
            }
            6 => {
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                let last = c.g[V1][0] == 0;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
                stage = if last { 8 } else { 7 };
            }
            7 => {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.cycles += 3;
                c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0 {
                    stage = 8;
                } else {
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    if back_edge_unwinds(c, RESUME) {
                        return;
                    }
                    stage = 4;
                }
            }
            _ => {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
        }
    }
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Spatial keys.

pub(crate) const SK_ENTRY: u32 = 0x0032_8f28;

/// Loop head (`level++`, halve the cell coordinates): resume entry.
pub(crate) const SK_GROW: u32 = 0x0032_9070;

pub(crate) const SK_END: u32 = 0x0032_91dc;

/// `void SpatialKey_FromAABB(SpatialKey* key, const AABB* box)` (0x328f28):
/// `key = {level, x, y, z}` of the smallest grid cell of size `2^level`
/// (level >= 11, from the largest extent scaled by a global) containing
/// the box with a margin (`gp-0x2df4`, in cells): start at the cell of the
/// box centre, then go up a level until the box fits. Dispatcher-shaped
/// in the original: re-entered at the loop head after an unwind.
pub fn spatial_key_from_aabb(c: &mut Cpu) {
    match c.pc {
        SK_ENTRY => spatial_key_initial(c),
        SK_GROW => spatial_key_grow(c),
        pc => {
            c.jump_out(pc, SK_ENTRY, SK_END);
            return;
        }
    }
    while !spatial_key_fits(c) {
        c.g[V1][0] = lw(c, r32(c, A2));
        if back_edge_unwinds(c, SK_GROW) {
            return;
        }
        spatial_key_grow(c);
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
    c.pc = ra;
}

/// Centre and extent (VU0), the start level and the cell of the centre.
pub(crate) fn spatial_key_initial(c: &mut Cpu) {
    c.cycles += 29;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
    c.g[A3][0] = c.g[A1][0];
    let aabb = r32(c, A3);
    let sp = sp(c);
    // extent = max - min
    c.lqc2(3, p(aabb, 0x10));
    c.lqc2(4, aabb);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp, 0x10));
    c.g[AT][0] = 0x3f00_0000;
    c.f[0] = 0x3f00_0000;
    c.lq(V0, p(sp, 0x10));
    c.g[A2][0] = c.g[A0][0];
    c.sq(V0, sp);
    c.g[V1][0] = sx(c.f[0]);
    // centre = min + extent * 0.5
    c.lqc2(4, sp);
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp, 0x30));
    c.lq(V0, p(sp, 0x30));
    c.sq(V0, p(sp, 0x20));
    c.lqc2(3, aabb);
    c.lqc2(4, p(sp, 0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp, 0x30));
    // The largest extent.
    let ey = fpu(c, p(sp, 4));
    let ex = fpu(c, sp);
    c.f[0] = ey.0;
    c.f[1] = ex.0;
    c.lq(V0, p(sp, 0x30));
    let wider = ex.lt(ey);
    abi::set_fcc(c, wider);
    c.sq(V0, p(sp, 0x10));
    let mut size = ex;
    if wider {
        c.cycles += 1;
        size = ey;
    }
    c.cycles += 5;
    let ez = fpu(c, p(sp, 8));
    c.f[0] = ez.0;
    let wider = size.lt(ez);
    abi::set_fcc(c, wider);
    if wider {
        size = ez;
    }
    // level = max(exponent(size * scale), 11)
    c.cycles += 9;
    let scale = fpu(c, gp(c, -0x2df8));
    c.f[0] = scale.0;
    let scaled = size * scale;
    c.f[1] = scaled.0;
    c.g[A0][0] = sx(scaled.0);
    c.g[V0][0] = sx(((scaled.0 as i32 >> 23) as u32 & 0xff).wrapping_sub(0x7f));
    c.g[V1][0] = ((c.g[V0][0] as i64) < 11) as u64;
    let key = r32(c, A2);
    sw_reg(c, key, V0);
    if c.g[V1][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0xb;
        sw_reg(c, key, V0);
    }
    // cell = floor(centre / 2^level)
    c.cycles += 13;
    c.g[V0][0] = lw(c, key);
    let cx = fpu(c, p(sp, 0x10));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7f) << 23);
    let cell = FpuF32(r32(c, V0));
    c.f[2] = cell.0;
    floor_div(c, cx, cell);
    c.cycles += 10;
    let cy = fpu(c, p(sp, 0x14));
    sw_reg(c, p(key, 4), V0);
    floor_div(c, cy, cell);
    c.cycles += 10;
    let cz = fpu(c, p(sp, 0x18));
    sw_reg(c, p(key, 8), V0);
    floor_div(c, cz, cell);
    c.cycles += 3;
    c.f[3] = fpu(c, gp(c, -0x2df4)).0;
    sw_reg(c, p(key, 0xc), V0);
}

/// `v0 = floor(v / cell)` as the original computes it (truncate, then one
/// less when the quotient is below the truncation).
pub(crate) fn floor_div(c: &mut Cpu, v: FpuF32, cell: FpuF32) {
    let q = v.div(cell);
    c.f[0] = q.0;
    let t = q.to_i32();
    c.g[V0][0] = sx(t as u32);
    let back = FpuF32::from_i32(t);
    c.f[1] = back.0;
    let below = q.lt(back);
    abi::set_fcc(c, below);
    if below {
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
    }
}

/// One level up: `level++`, cell coordinates halved (arithmetic shift),
/// stored through the stack as the original's unaligned doubleword copy.
pub(crate) fn spatial_key_grow(c: &mut Cpu) {
    c.cycles += 27;
    let key = r32(c, A2);
    let sp = sp(c);
    let sra1 = |v: u64| sx(((v as u32) as i32 >> 1) as u32);
    c.g[A0][0] = lw(c, p(key, 4));
    c.g[A1][0] = lw(c, p(key, 8));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
    c.g[V0][0] = lw(c, p(key, 0xc));
    c.g[A0][0] = sra1(c.g[A0][0]);
    c.g[A1][0] = sra1(c.g[A1][0]);
    sw_reg(c, p(sp, 0x30), V1);
    c.g[V0][0] = sra1(c.g[V0][0]);
    sw_reg(c, p(sp, 0x34), A0);
    sw_reg(c, p(sp, 0x38), A1);
    sw_reg(c, p(sp, 0x3c), V0);
    let lo = Addr(p(sp, 0x30)).read_u64_unaligned(c);
    c.g[V0][0] = lo;
    let hi = Addr(p(sp, 0x38)).read_u64_unaligned(c);
    c.g[V1][0] = hi;
    Addr(p(sp, 0x20)).write_u64_unaligned(c, lo);
    Addr(p(sp, 0x28)).write_u64_unaligned(c, hi);
    let lo = Addr(p(sp, 0x20)).read_u64_unaligned(c);
    c.g[V0][0] = lo;
    let hi = Addr(p(sp, 0x28)).read_u64_unaligned(c);
    c.g[V1][0] = hi;
    Addr(key).write_u64_unaligned(c, lo);
    Addr(p(key, 8)).write_u64_unaligned(c, hi);
}

/// One bound of the fit test: `f0 = (cell -/+ margin) * size` against
/// `f1 = [bound_at]` (`c.le.s`).
pub(crate) fn key_bound(c: &mut Cpu, cell: u32, bound_at: u32, upper: bool) -> bool {
    let bound = fpu(c, bound_at);
    c.f[1] = bound.0;
    let corner = FpuF32::from_i32(cell as i32);
    let margin = FpuF32(c.f[3]);
    let size = FpuF32(c.f[2]);
    let edge = (if upper {
        corner + margin
    } else {
        corner - margin
    }) * size;
    c.f[0] = edge.0;
    let holds = if upper {
        bound.le(edge)
    } else {
        edge.le(bound)
    };
    abi::set_fcc(c, holds);
    holds
}

/// Does the box fit the key's cell (with the margin)? Leaves t0 = the
/// answer and the scratch registers of the original.
pub(crate) fn spatial_key_fits(c: &mut Cpu) -> bool {
    c.cycles += 14;
    let key = r32(c, A2);
    let aabb = r32(c, A3);
    c.g[A1][0] = lw(c, p(key, 4));
    c.g[V0][0] = lw(c, key);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7f) << 23);
    c.f[2] = r32(c, V0);
    c.g[T0][0] = 0;
    let fits = key_bound(c, r32(c, A1), aabb, false)
        && {
            c.cycles += 9;
            c.g[V1][0] = lw(c, p(key, 8));
            key_bound(c, r32(c, V1), p(aabb, 4), false)
        }
        && {
            c.cycles += 10;
            c.g[A0][0] = lw(c, p(key, 0xc));
            let ok = key_bound(c, r32(c, A0), p(aabb, 8), false);
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(1));
            ok
        }
        && {
            c.cycles += 9;
            let ok = key_bound(c, r32(c, V0), p(aabb, 0x10), true);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            ok
        }
        && {
            c.cycles += 9;
            let ok = key_bound(c, r32(c, V1), p(aabb, 0x14), true);
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
            ok
        }
        && {
            c.cycles += 9;
            key_bound(c, r32(c, V0), p(aabb, 0x18), true)
        };
    if fits {
        c.g[T0][0] = 1;
    }
    c.cycles += 2;
    fits
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
// Spatial hash.

pub(crate) const SH_ENTRY: u32 = 0x0032_91e0;

pub(crate) const SH_END: u32 = 0x0032_958c;

/// Resume entries (event checks in the bucket-emptiness scans): "not
/// empty" and the scan of the last three words, after a cell change and
/// after a re-insert in the same root.
pub(crate) const SH_MOVED_NONEMPTY: u32 = 0x0032_9358;

pub(crate) const SH_MOVED_TAIL: u32 = 0x0032_9368;

pub(crate) const SH_SAME_NONEMPTY: u32 = 0x0032_94f0;

pub(crate) const SH_SAME_TAIL: u32 = 0x0032_9500;

/// Where a bucket-emptiness scan starts (or resumes).
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Probe {
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

pub(crate) fn spatial_hash_update_entry(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn sh_moved_root(c: &mut Cpu, frame: Frame, probe: Probe) -> Flow {
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
pub(crate) fn sh_same_root(c: &mut Cpu, frame: Frame) -> Flow {
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
pub(crate) fn sh_reinsert(c: &mut Cpu, frame: Frame, probe: Probe) -> Flow {
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
pub(crate) fn bucket_empty(
    c: &mut Cpu,
    mut probe: Probe,
    nonempty_head: u32,
    tail_head: u32,
) -> bool {
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
pub(crate) fn sh_exit(c: &mut Cpu, frame: Frame) {
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
// RollerModifier_CopyMatrixBlock (0x0000327cc8).

/// `RollerModifier_CopyMatrixBlock`: copy eight quadwords plus a 0xc0-byte matrix block, using unaligned load/store pairs when needed.
pub fn roller_modifier_copy_matrix_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[A2][0] = c.g[A0][0];
        c.g[V1][0] = c.g[A1][0];
        c.g[A3][0] = 0x7;
        'l_327cd8: loop {
            c.cycles += 7;
            c.lq(2, (c.g[V1][0] as u32));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
            c.sq(2, (c.g[A2][0] as u32));
            let t = (c.g[A3][0] as i64) >= 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00327cd8);
                }
                continue 'l_327cd8;
            } else {
                'b_327da0: {
                    'b_327d6c: {
                        c.cycles += 8;
                        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc0));
                        c.lq(3, (c.g[A1][0] as u32).wrapping_add(0x80));
                        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc0));
                        c.g[T0][0] = c.g[A3][0];
                        c.g[V0][0] = c.g[A2][0] | c.g[A3][0];
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x80));
                        if t {
                            break 'b_327d6c;
                        } else {
                            c.cycles += 1;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x140));
                            'l_327d18: loop {
                                c.cycles += 19;
                                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                                c.ldr(2, (c.g[A2][0] as u32));
                                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
                                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
                                c.ldl(9, (c.g[A2][0] as u32).wrapping_add(0x17));
                                c.ldr(9, (c.g[A2][0] as u32).wrapping_add(0x10));
                                c.ldl(10, (c.g[A2][0] as u32).wrapping_add(0x1f));
                                c.ldr(10, (c.g[A2][0] as u32).wrapping_add(0x18));
                                c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                                c.sdr(2, (c.g[A3][0] as u32));
                                c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
                                c.sdr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                                c.sdl(9, (c.g[A3][0] as u32).wrapping_add(0x17));
                                c.sdr(9, (c.g[A3][0] as u32).wrapping_add(0x10));
                                c.sdl(10, (c.g[A3][0] as u32).wrapping_add(0x1f));
                                c.sdr(10, (c.g[A3][0] as u32).wrapping_add(0x18));
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                let t = c.g[A2][0] != c.g[A1][0];
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00327d18);
                                    }
                                    continue 'l_327d18;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x98), c.g[T0][0]);
                                    if t {
                                        break 'b_327da0;
                                    } else {
                                        break 'b_327d6c;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x140));
                    'l_327d70: loop {
                        c.cycles += 11;
                        c.g[V0][0] = c.ld((c.g[A2][0] as u32));
                        c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                        c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                        c.sd((c.g[A3][0] as u32), c.g[V0][0]);
                        c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                        c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        let t = c.g[A2][0] != c.g[A1][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00327d70);
                            }
                            continue 'l_327d70;
                        } else {
                            c.cycles += 1;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x98), c.g[T0][0]);
                            break 'b_327da0;
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = c.g[A0][0];
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    });
}

/// `int SphereTree_CollideTriangle2Sided(void* tree, const Vec4* p, const
/// Vec4* dir, ...)` (0x328030): the two-sided triangle test: `dot` of the
/// direction from the tree's centre (`+0x80`) to `p` against `dir`; when the
/// tree's radius is smaller than |dot| there is no hit, otherwise the
/// collision runs with the direction (or its negation when `dot < 0`).
pub fn sphere_tree_collide_triangle2_sided(c: &mut Cpu) {
    'b_328140: {
        c.cycles += 39;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.sq(17, c.pa(SP, 0x80));
        c.sq(18, c.pa(SP, 0x70));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, c.pa(SP, 0x90));
        c.g[S2][0] = c.g[A1][0];
        c.sq(19, c.pa(SP, 0x60));
        c.g[S0][0] = c.g[T0][0];
        c.sq(20, c.pa(SP, 0x50));
        c.g[S3][0] = c.g[A2][0];
        c.sq(21, c.pa(SP, 0x40));
        c.g[S4][0] = c.g[A3][0];
        c.sq(22, c.pa(SP, 0x30));
        c.g[S5][0] = c.g[T1][0];
        c.std(SP, 0x20, c.g[RA][0]);
        c.g[S6][0] = c.g[T2][0];
        c.swc1(20, c.pa(SP, 0xa0));
        c.lqc2(3, c.pa(S1, 0x80));
        c.lqc2(4, c.pa(S2, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, c.pa(SP, 0x10));
        c.lq(2, c.pa(SP, 0x10));
        c.sq(2, c.pa(SP, 0));
        c.lqc2(3, c.pa(S0, 0));
        c.lqc2(5, c.pa(SP, 0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 20);
        c.g[RA][0] = sx(0x003280b8);
        c.pc = 0x0032c590;
        call_indirect(c, 0x0032c590);
        if c.pc != 0x003280b8 {
            return;
        }
        c.abs_s(1, 20);
        c.c_lt_s(0, 1);
        let t = c.fcc();
        c.g[V0][0] = 0u64;
        if t {
            break 'b_328140;
        } else {
            'b_328138: {
                'b_3280fc: {
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_lt_s(20, 0);
                    let t = c.fcc();
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        break 'b_3280fc;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A3][0] = c.g[S4][0];
                        c.g[T0][0] = c.g[S0][0];
                        c.g[T1][0] = c.g[S5][0];
                        let t = true;
                        c.g[T2][0] = c.g[S6][0];
                        if t {
                            break 'b_328138;
                        } else {
                            break 'b_3280fc;
                        }
                    }
                }
                c.cycles += 15;
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 0);
                c.mfc1(2, 0);
                c.lqc2(4, c.pa(S0, 0));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, c.pa(SP, 0x10));
                c.g[A1][0] = c.g[S2][0];
                c.lq(2, c.pa(SP, 0x10));
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[T1][0] = c.g[S5][0];
                c.sq(2, c.pa(SP, 0));
                c.g[T2][0] = c.g[S6][0];
                c.g[T0][0] = c.g[SP][0];
                break 'b_328138;
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x00328140);
            c.pc = 0x0032ca78;
            call_indirect(c, 0x0032ca78);
            if c.pc != 0x00328140 {
                return;
            }
            break 'b_328140;
        }
    }
    c.cycles += 11;
    c.lq(16, c.pa(SP, 0x90));
    c.lq(17, c.pa(SP, 0x80));
    c.lq(18, c.pa(SP, 0x70));
    c.lq(19, c.pa(SP, 0x60));
    c.lq(20, c.pa(SP, 0x50));
    c.lq(21, c.pa(SP, 0x40));
    c.lq(22, c.pa(SP, 0x30));
    c.g[RA][0] = c.ldd(SP, 0x20);
    c.lwc1(20, c.pa(SP, 0xa0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
    c.pc = ra;
    return;
}

/// `int cPS2ChunkResolve_ParseHeader(ChunkResolve* self, char* buf, void* ctx)`
/// (0x327da8): copy a chunk header into the resolve block, lay the payload
/// pointers out after it, and return the header size.
pub fn c_ps2_chunk_resolve_parse_header(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0032_7df0;
        /// A bulk copy (`func_003E6574(dst, src, n)`).
        const COPY: u32 = 0x003e_6574;
        c.cycles += 8;
        let f = Frame::open(c, 0x50);
        f.save(c, S0, 0x40);
        f.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        f.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A1][0];
        let buf = c.g[S1][0];
        c.g[V0][0] = 0;
        if buf != 0 {
            c.cycles += 61;
            let obj = c.g[S0][0] as u32;
            let blk = obj.wrapping_add(0xc0);
            let ctx = c.g[A2][0];
            c.g[V0][0] = sx(blk);
            c.g[V1][0] = 1;
            c.sw(blk, ctx);
            c.g[A0][0] = sx(f.at(c, 0x10));
            c.sw(obj.wrapping_add(0xc4), 0);
            c.g[A1][0] = c.g[S1][0];
            c.sw(obj.wrapping_add(0x98), c.g[V0][0]);
            c.g[A2][0] = 4;
            c.g[RA][0] = sx(RET);
            c.sw(obj.wrapping_add(0xd0), c.g[V1][0]);
            c.pc = COPY;
            ssx3_ee_rt::funcs::call_indirect(c, COPY);
            if c.pc != RET {
                return Err(crate::call::Unwound);
            }
            // The two lengths go to the stack first ...
            c.g[A0][0] = sx(f.at(c, 0x10));
            c.g[A1][0] = sx(buf as u32);
            c.g[A2][0] = 4;
            call_ptr(c, COPY, 0x0032_7e00)?;
            c.g[A0][0] = sx(f.at(c, 0x14));
            c.g[A1][0] = sx((buf as u32).wrapping_add(4));
            c.g[A2][0] = 4;
            call_ptr(c, COPY, 0x0032_7e14)?;
            // ... then the fixed arrays into the resolve block.
            for (dst_off, src_off, n, ret) in [
                (8u32, 8u32, 4u64, 0x0032_7e14u32),
                (0xc, 0xc, 4, 0x0032_7e28),
                (0x14, 0x10, 0xc, 0x0032_7e3c),
                (0x2c, 0x1c, 0xc, 0x0032_7e50),
                (0x38, 0x28, 0x24, 0x0032_7e64),
                (0x5c, 0x4c, 0x24, 0x0032_7e78),
            ] {
                let b = Addr(obj.wrapping_add(0x98)).lw(c);
                c.g[A0][0] = sx((b as u32).wrapping_add(dst_off));
                c.g[A1][0] = sx((buf as u32).wrapping_add(src_off));
                c.g[A2][0] = n;
                call_ptr(c, COPY, ret)?;
            }
            c.g[V0][0] = Addr(obj.wrapping_add(0x98)).lw(c);
            c.g[A1][0] = sx((buf as u32).wrapping_add(0x70));
            c.g[A0][0] = 0xc;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
            c.g[V1][0] = Addr(obj.wrapping_add(0x98)).lw(c);
            let blkp = c.g[V1][0] as u32;
            c.g[V0][0] = sx((Addr(blkp.wrapping_add(0xc)).lw(c) as u32).wrapping_add(1));
            c.mult(0, A2, V0, A0); // a2 = (count + 1) * 12 (a0 = 12 at this point)
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.sw(blkp.wrapping_add(0x24), c.g[A1][0]);
            c.g[A2][0] = Addr(obj.wrapping_add(0x98)).lw(c);
            let len1 = Addr(f.at(c, 0x14)).lw(c);
            let len0 = Addr(f.at(c, 0x10)).lw(c);
            c.g[A0][0] = len0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((len1 as u32)));
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
            let t = c.g[V1][0] == 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((len0 as u32)));
            let blkp = c.g[A2][0] as u32;
            if t {
                c.cycles += 2;
                c.g[V0][0] = Addr(blkp.wrapping_add(0x24)).lw(c);
                c.sw(blkp.wrapping_add(0x28), c.g[V0][0]);
            } else {
                c.cycles += 2;
                c.sw(blkp.wrapping_add(0x28), 0);
            }
            c.cycles += 13;
            let blkp = Addr(obj.wrapping_add(0x98)).lw(c) as u32;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[S1][0] as u32)));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.lwc1(0, blkp.wrapping_add(0x14));
            c.swc1(0, f.at(c, 0));
            c.lwc1(1, blkp.wrapping_add(0x18));
            c.swc1(1, f.at(c, 4));
            c.lwc1(0, blkp.wrapping_add(0x1c));
            c.swc1(2, f.at(c, 0xc));
            c.swc1(0, f.at(c, 8));
            c.lq(3, f.at(c, 0));
            c.sq(3, obj.wrapping_add(0x80));
        }
        c.cycles += 5;
        c.lq(16, f.at(c, 0x40));
        c.lq(17, f.at(c, 0x30));
        f.restore_ra(c, 0x20);
        f.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AIFwdDiffCache_Get (0x003279d0)
// ---------------------------------------------------------------------------

/// `AIFwdDiffCache` fields: the least recently used node, the hash bucket
/// array and the bucket count.
pub(crate) const FDC_LRU: u32 = 0x4;

pub(crate) const FDC_BUCKETS: u32 = 0x8;

pub(crate) const FDC_COUNT: u32 = 0xc;

/// Cache node fields: the key, the LRU ring's next/prev, the bucket chain.
pub(crate) const FDN_KEY: u32 = 0x0;

pub(crate) const FDN_LRU_NEXT: u32 = 0x4;

pub(crate) const FDN_HASH_NEXT: u32 = 0xc;

/// The source spline's cache key.
pub(crate) const FD_SRC_KEY: u32 = 0x150;

pub(crate) const FDC_TOUCH: Callee = Callee {
    addr: 0x0032_7ac0,
    f: ee::s10::f_00327ac0,
};

pub(crate) const FDC_HASH_REMOVE: Callee = Callee {
    addr: 0x0032_7b30,
    f: ee::s10::f_00327b30,
};

pub(crate) const FDC_HASH_INSERT: Callee = Callee {
    addr: 0x0032_7bb0,
    f: ee::s10::f_00327bb0,
};

pub(crate) const FDC_BUILD_ENTRY: Callee = Callee {
    addr: 0x0032_7c00,
    f: ee::s10::f_00327c00,
};

/// `void AIFwdDiffCache_Get(AIFwdDiffCache* self, const void* src,
/// Node** out)` (0x003279d0): the forward-difference table for the AI
/// spline `src`, cached by its key (`src+0x150`). The key's bucket chain
/// (sorted ascending) is searched; a hit is moved to the front of the LRU
/// ring, a miss recycles the least recently used node (rehashed under the
/// new key, the ring advanced, the table rebuilt from `src`). `*out` is the
/// node.
pub fn ai_fwd_diff_cache_get(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S1, 0x40);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = r32(c, S2);
        c.g[V0][0] = Addr(this.wrapping_add(FDC_COUNT)).lw(c);
        c.g[S1][0] = Addr(r32(c, S4).wrapping_add(FD_SRC_KEY)).lw(c);

        // 0x00327a08: the key's bucket.
        c.cycles += 7;
        let (_, bucket) = abi::divu(c, r32(c, S1), r32(c, V0));
        c.g[A0][0] = Addr(this.wrapping_add(FDC_BUCKETS)).lw(c);
        c.g[V1][0] = sx((bucket << 2).wrapping_add(r32(c, A0)));
        c.g[S0][0] = Addr(r32(c, V1)).lw(c);

        // 0x00327a44: walk the chain while node->key <= key.
        loop {
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                c.g[S0][0] = Addr(this.wrapping_add(FDC_LRU)).lw(c);
                break;
            }
            c.cycles += 5;
            let node_key = Addr(r32(c, S0).wrapping_add(FDN_KEY)).lw(c);
            c.g[V1][0] = node_key;
            c.g[V0][0] = (c.g[S1][0] < node_key) as u64;
            if c.g[V0][0] != 0 {
                // 0x00327a60: past the key, a miss.
                c.cycles += 1;
                c.g[S0][0] = Addr(this.wrapping_add(FDC_LRU)).lw(c);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_7a28);
            }
            // 0x00327a28
            c.cycles += 2;
            if c.g[V1][0] != c.g[S1][0] {
                c.g[S0][0] = Addr(r32(c, S0).wrapping_add(FDN_HASH_NEXT)).lw(c);
                continue;
            }
            // 0x00327a30: a hit.
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            FDC_TOUCH.call(c, 0x0032_7a3c)?;
            Addr(r32(c, S3)).sw(c, c.g[S0][0]);
            epilogue(c, frame);
            return Ok(());
        }

        // 0x00327a64: recycle the least recently used node.
        c.cycles += 14;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        FDC_HASH_REMOVE.call(c, 0x0032_7a70)?;
        let node = r32(c, S0);
        Addr(node.wrapping_add(FDN_KEY)).sw(c, c.g[S1][0]);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        FDC_HASH_INSERT.call(c, 0x0032_7a80)?;
        c.g[V0][0] = Addr(node.wrapping_add(FDN_LRU_NEXT)).lw(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S0][0];
        Addr(this.wrapping_add(FDC_LRU)).sw(c, c.g[V0][0]);
        FDC_BUILD_ENTRY.call(c, 0x0032_7a98)?;
        Addr(r32(c, S3)).sw(c, c.g[S0][0]);
        epilogue(c, frame);
        Ok(())
    });

    /// 0x00327a9c
    fn epilogue(c: &mut Cpu, frame: Frame) {
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
    }
}

/// `bool Collide_SphereVsBoxEdges(const Vec4* min, const Vec4* max, const
/// Vec4* centre, Vec4* point, Vec4* normal, float* dist, float r)`
/// (0x329590, asm only in the reference): a sphere against an axis-aligned
/// box's faces.
///
/// For each axis in turn (x, y, z), when the centre lies inside the box's
/// extent on the other two axes and the sphere reaches the face pair
/// (`centre - min + r >= 0` and `max - centre + r >= 0`), the nearer face
/// is taken if it beats the best distance so far (`*dist`, starting at the
/// `.lit4` 1e30): `point` is the centre snapped onto that face and
/// `normal` the axis unit vector (table at 0x4ff140) for the max face, or
/// its negation (VU0 `vmul` by -1) for the min face. Returns whether any
/// face was hit (`*dist` no longer 1e30).
///
/// A leaf; the body keeps the original's block structure (every cycle
/// charge, the FPU temporaries f0-f5, VF3-VF5, v1 and the 0x20-byte stack
/// scratch the negated normal goes through).
pub fn collide_sphere_vs_box_edges(c: &mut Cpu) {
    'b_3296b8: {
        'b_3296b4: {
            c.cycles += 8;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x2df0));
            c.swc1(5, (c.g[T1][0] as u32));
            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.c_le_s(0, 1);
            let t = !c.fcc();
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            if t {
                break 'b_3296b4;
            } else {
                c.cycles += 5;
                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.c_le_s(1, 0);
                if !c.fcc() {
                    c.lwc1(1, (c.g[A2][0] as u32));
                    break 'b_3296b8;
                } else {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.c_le_s(0, 1);
                    if !c.fcc() {
                        c.lwc1(1, (c.g[A2][0] as u32));
                        break 'b_3296b8;
                    } else {
                        c.cycles += 5;
                        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.c_le_s(1, 0);
                        if !c.fcc() {
                            c.lwc1(1, (c.g[A2][0] as u32));
                            break 'b_3296b8;
                        } else {
                            c.cycles += 11;
                            c.lwc1(0, (c.g[A2][0] as u32));
                            c.lwc1(1, (c.g[A0][0] as u32));
                            c.lwc1(2, (c.g[A1][0] as u32));
                            c.sub_s(1, 0, 1);
                            c.mtc1(0, 4);
                            c.sub_s(2, 2, 0);
                            c.add_s(3, 1, 12);
                            c.c_le_s(4, 3);
                            let t = !c.fcc();
                            c.add_s(2, 2, 12);
                            if t {
                                break 'b_3296b4;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(4, 2);
                                if !c.fcc() {
                                    c.lwc1(1, (c.g[A2][0] as u32));
                                    break 'b_3296b8;
                                } else {
                                    'b_329684: {
                                        c.cycles += 4;
                                        c.c_lt_s(3, 5);
                                        if !c.fcc() {
                                            c.lwc1(0, (c.g[T1][0] as u32));
                                            break 'b_329684;
                                        } else {
                                            c.cycles += 18;
                                            c.lq(2, (c.g[A2][0] as u32));
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 1);
                                            c.sq(2, (c.g[A3][0] as u32));
                                            c.g[V0][0] = 0x500000;
                                            c.lwc1(0, (c.g[A0][0] as u32));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
                                            c.swc1(0, (c.g[A3][0] as u32));
                                            c.swc1(3, (c.g[T1][0] as u32));
                                            c.mfc1(3, 1);
                                            c.lqc2(4, (c.g[V0][0] as u32));
                                            c.qmtc2(3, 3);
                                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.sq(2, (c.g[T0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32));
                                            c.lwc1(0, (c.g[T1][0] as u32));
                                            break 'b_329684;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.c_lt_s(2, 0);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0x500000;
                                    if t {
                                        break 'b_3296b4;
                                    } else {
                                        c.cycles += 8;
                                        c.lq(2, (c.g[A2][0] as u32));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xec0));
                                        c.sq(2, (c.g[A3][0] as u32));
                                        c.lwc1(0, (c.g[A1][0] as u32));
                                        c.swc1(0, (c.g[A3][0] as u32));
                                        c.swc1(2, (c.g[T1][0] as u32));
                                        c.lq(2, (c.g[V1][0] as u32));
                                        c.sq(2, (c.g[T0][0] as u32));
                                        break 'b_3296b4;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.lwc1(1, (c.g[A2][0] as u32));
        break 'b_3296b8;
    }
    'b_3297d4: {
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32));
        c.c_le_s(0, 1);
        if !c.fcc() {
            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
            break 'b_3297d4;
        } else {
            c.cycles += 5;
            c.lwc1(0, (c.g[A1][0] as u32));
            c.c_le_s(1, 0);
            if !c.fcc() {
                c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                break 'b_3297d4;
            } else {
                c.cycles += 6;
                c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.c_le_s(0, 1);
                if !c.fcc() {
                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                    break 'b_3297d4;
                } else {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.c_le_s(1, 0);
                    if !c.fcc() {
                        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                        break 'b_3297d4;
                    } else {
                        'b_3297d0: {
                            c.cycles += 11;
                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.sub_s(1, 0, 1);
                            c.mtc1(0, 4);
                            c.sub_s(2, 2, 0);
                            c.add_s(3, 1, 12);
                            c.c_le_s(4, 3);
                            let t = !c.fcc();
                            c.add_s(2, 2, 12);
                            if t {
                                break 'b_3297d0;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(4, 2);
                                if !c.fcc() {
                                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_3297d4;
                                } else {
                                    'b_3297a0: {
                                        c.cycles += 4;
                                        c.lwc1(0, (c.g[T1][0] as u32));
                                        c.c_lt_s(3, 0);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_3297a0;
                                        } else {
                                            c.cycles += 18;
                                            c.lq(2, (c.g[A2][0] as u32));
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 1);
                                            c.sq(2, (c.g[A3][0] as u32));
                                            c.g[V0][0] = 0x500000;
                                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0xeb0));
                                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                                            c.swc1(3, (c.g[T1][0] as u32));
                                            c.mfc1(3, 1);
                                            c.lqc2(4, (c.g[V0][0] as u32));
                                            c.qmtc2(3, 3);
                                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.sq(2, (c.g[T0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32));
                                            c.lwc1(0, (c.g[T1][0] as u32));
                                            break 'b_3297a0;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.c_lt_s(2, 0);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0x500000;
                                    if t {
                                        break 'b_3297d0;
                                    } else {
                                        c.cycles += 8;
                                        c.lq(2, (c.g[A2][0] as u32));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeb0));
                                        c.sq(2, (c.g[A3][0] as u32));
                                        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                                        c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                                        c.swc1(2, (c.g[T1][0] as u32));
                                        c.lq(2, (c.g[V1][0] as u32));
                                        c.sq(2, (c.g[T0][0] as u32));
                                        break 'b_3297d0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                        break 'b_3297d4;
                    }
                }
            }
        }
    }
    'b_3298f0: {
        c.cycles += 5;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
        c.c_le_s(0, 1);
        if !c.fcc() {
            c.lwc1(1, (c.g[T1][0] as u32));
            break 'b_3298f0;
        } else {
            c.cycles += 5;
            c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
            c.c_le_s(1, 0);
            if !c.fcc() {
                c.lwc1(1, (c.g[T1][0] as u32));
                break 'b_3298f0;
            } else {
                c.cycles += 6;
                c.lwc1(1, (c.g[A2][0] as u32));
                c.lwc1(0, (c.g[A0][0] as u32));
                c.c_le_s(0, 1);
                if !c.fcc() {
                    c.lwc1(1, (c.g[T1][0] as u32));
                    break 'b_3298f0;
                } else {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[A1][0] as u32));
                    c.c_le_s(1, 0);
                    if !c.fcc() {
                        c.lwc1(1, (c.g[T1][0] as u32));
                        break 'b_3298f0;
                    } else {
                        'b_3298ec: {
                            c.cycles += 11;
                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sub_s(1, 0, 1);
                            c.mtc1(0, 4);
                            c.sub_s(2, 2, 0);
                            c.add_s(3, 1, 12);
                            c.c_le_s(4, 3);
                            let t = !c.fcc();
                            c.add_s(2, 2, 12);
                            if t {
                                break 'b_3298ec;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(4, 2);
                                if !c.fcc() {
                                    c.lwc1(1, (c.g[T1][0] as u32));
                                    break 'b_3298f0;
                                } else {
                                    'b_3298bc: {
                                        c.cycles += 4;
                                        c.lwc1(0, (c.g[T1][0] as u32));
                                        c.c_lt_s(3, 0);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_3298bc;
                                        } else {
                                            c.cycles += 18;
                                            c.lq(2, (c.g[A2][0] as u32));
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 1);
                                            c.sq(2, (c.g[A3][0] as u32));
                                            c.g[V0][0] = 0x500000;
                                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
                                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.swc1(3, (c.g[T1][0] as u32));
                                            c.mfc1(3, 1);
                                            c.lqc2(4, (c.g[V0][0] as u32));
                                            c.qmtc2(3, 3);
                                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            c.sq(2, (c.g[T0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32));
                                            c.lwc1(0, (c.g[T1][0] as u32));
                                            break 'b_3298bc;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.c_lt_s(2, 0);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0x500000;
                                    if t {
                                        break 'b_3298ec;
                                    } else {
                                        c.cycles += 8;
                                        c.lq(2, (c.g[A2][0] as u32));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
                                        c.sq(2, (c.g[A3][0] as u32));
                                        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                                        c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.swc1(2, (c.g[T1][0] as u32));
                                        c.lq(2, (c.g[V1][0] as u32));
                                        c.sq(2, (c.g[T0][0] as u32));
                                        break 'b_3298ec;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(1, (c.g[T1][0] as u32));
                        break 'b_3298f0;
                    }
                }
            }
        }
    }
    'b_329908: {
        c.cycles += 5;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2dec));
        c.c_eq_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x1;
        if t {
            break 'b_329908;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_329908;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
    return;
}

pub(crate) mod oc230_shm_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const REMOVE: Callee = Callee {
        addr: 0x0032_84b8,
        f: ee::s10::SpatialHash_Remove,
    };

    pub const INSERT_AT_KEY: Callee = Callee {
        addr: 0x0032_8660,
        f: ee::s10::SpatialHash_InsertAtKey,
    };

    pub const MOVE: Callee = Callee {
        addr: 0x0032_8808,
        f: ee::s10::SpatialHash_Move,
    };

    pub const DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };

    pub const ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    };

    /// `memset` (another shard: through the function table).
    pub const MEMSET: u32 = 0x003e_6448;
}

/// `reg + off` (the low word of a GPR as a base address).
#[inline(always)]
pub(crate) fn oc230_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `SpatialHash_Move`'s entry, its end, and the loop heads it is re-entered
/// at after an unwind (the bucket scan's "not empty" and tail loop, the
/// new bucket's clear loop).
pub(crate) const SHM_ENTRY: u32 = 0x0032_8808;

pub(crate) const SHM_END: u32 = 0x0032_8c20;

pub(crate) const SHM_NONEMPTY: u32 = 0x0032_8ac0;

pub(crate) const SHM_TAIL: u32 = 0x0032_8af0;

pub(crate) const SHM_CLEAR: u32 = 0x0032_8ba8;

/// Where the emptiness scan of the old child's bucket array starts (or
/// resumes).
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Oc230Scan {
    /// The eight unrolled words (right after `SpatialHash_Remove`).
    Words,
    /// A non-zero word was found in the tail loop.
    NonEmpty,
    /// The loop over the last three words (a0 counter, v1 pointer).
    Tail,
}

/// `sx(a + b)` of two GPR low words.
#[inline(always)]
pub(crate) fn oc230_add(c: &Cpu, a: usize, b: usize) -> u64 {
    sx(r32(c, a).wrapping_add(r32(c, b)))
}

/// `sx(reg << n)`.
#[inline(always)]
pub(crate) fn oc230_sll(c: &Cpu, reg: usize, n: u32) -> u64 {
    sx(r32(c, reg) << n)
}

/// `sx(reg >> n)` (arithmetic, `srav`).
#[inline(always)]
pub(crate) fn oc230_sra(c: &Cpu, reg: usize, n: usize) -> u64 {
    sx(((r32(c, reg) as i32) >> (r32(c, n) & 31)) as u32)
}

/// `SpatialHash_Move` epilogue (s0 restored before).
pub(crate) fn oc230_shm_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0xb0);
    frame.restore(c, S2, 0xa0);
    frame.restore(c, S3, 0x90);
    frame.restore(c, S4, 0x80);
    frame.restore(c, S5, 0x70);
    frame.restore(c, S6, 0x60);
    frame.restore(c, S7, 0x50);
    frame.restore(c, FP, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
}

/// `void SpatialHash_Move(SpatialHash* h /*a0*/, int key /*a1*/, Link* e
/// /*a2*/, const Key* old /*a3*/, const Key* new /*t0*/, const Key* cell
/// /*t1*/)` (0x328808): moves entry e of an octree cell (level, x, y, z)
/// from the old key to the new one. New key at the cell's level: remove
/// and push e on the bucket list `h->buckets[key]` (+0x20). Old key at the
/// cell's level: unlink e from its bucket and insert it at the new key.
/// Otherwise both keys are in child cells: the same child recurses with
/// the child key (level - 1, coordinates 2 * cell + octant); different
/// children: remove from the old child (freeing its hash when all eleven
/// bucket words are empty), then insert into the new child, allocating
/// and clearing its hash (0x2c bytes) when absent. Dispatcher-shaped in the
/// original: re-entered at the scan and clear loop heads after an unwind.
pub fn spatial_hash_move(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xd0 };
        match c.pc {
            SHM_ENTRY => oc230_shm_entry(c, frame),
            SHM_NONEMPTY => oc230_shm_after_remove(c, frame, Oc230Scan::NonEmpty),
            SHM_TAIL => oc230_shm_after_remove(c, frame, Oc230Scan::Tail),
            SHM_CLEAR => oc230_shm_clear_insert(c, frame),
            pc => {
                c.jump_out(pc, SHM_ENTRY, SHM_END);
                Ok(())
            }
        }
    })
}

pub(crate) fn oc230_shm_entry(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    use oc230_shm_callee as callee;
    c.cycles += 20;
    Frame::open(c, 0xd0);
    c.g[T3][0] = c.g[A3][0];
    frame.save(c, S0, 0xc0);
    frame.save(c, S2, 0xa0);
    c.g[S0][0] = c.g[T1][0];
    frame.save(c, S3, 0x90);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S7, 0x50);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, FP, 0x40);
    frame.save(c, S1, 0xb0);
    c.g[FP][0] = c.g[T0][0];
    frame.save(c, S4, 0x80);
    frame.save(c, S5, 0x70);
    frame.save(c, S6, 0x60);
    frame.save_ra(c, 0x30);
    c.g[V1][0] = c.lw(r32(c, FP));
    c.g[T4][0] = c.lw(r32(c, S0));
    c.g[S7][0] = c.g[A1][0];
    let entry = r32(c, S2);

    if c.g[V1][0] == c.g[T4][0] {
        // The new key is at the cell's level: remove, push on the bucket.
        c.cycles += 8;
        c.g[T0][0] = c.g[S0][0];
        callee::REMOVE.call(c, 0x0032_8860)?;
        c.g[V0][0] = oc230_sll(c, S7, 2);
        c.g[A0][0] = oc230_add(c, V0, S3);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x20));
        let head = r32(c, A0);
        c.g[V0][0] = c.lw(head);
        if c.g[V0][0] != 0 {
            let v = c.g[S2][0];
            c.sw(oc230_at(c, V0, 4), v);
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw(head);
        c.sw(entry.wrapping_add(4), 0);
        let v = c.g[V0][0];
        c.sw(entry, v);
        let v = c.g[S2][0];
        c.sw(head, v);
        c.cycles += 1;
        frame.restore(c, S0, 0xc0);
        oc230_shm_exit(c, frame);
        return Ok(());
    }

    c.cycles += 3;
    c.g[V0][0] = c.lw(r32(c, T3));
    let old_at_cell = c.g[V0][0] == c.g[T4][0];
    c.g[A2][0] = sx(r32(c, T4).wrapping_sub(r32(c, V0)));
    if old_at_cell {
        // The old key is at the cell's level: unlink from its bucket,
        // insert at the new key.
        c.cycles += 5;
        c.g[V0][0] = oc230_sll(c, S7, 2);
        c.g[A0][0] = c.lw(entry);
        c.g[V1][0] = oc230_add(c, V0, S3);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x20));
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(entry.wrapping_add(4));
            let v = c.g[V0][0];
            c.sw(oc230_at(c, A0, 4), v);
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(entry.wrapping_add(4));
        c.g[V0][0] = c.lw(entry);
        let v = c.g[V0][0];
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.sw(r32(c, A0), v);
        } else {
            c.cycles += 1;
            c.sw(r32(c, V1), v);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[FP][0];
        c.g[T0][0] = c.g[S0][0];
        callee::INSERT_AT_KEY.call(c, 0x0032_88e4)?;
        frame.restore(c, S0, 0xc0);
        oc230_shm_exit(c, frame);
        return Ok(());
    }

    // Both keys below the cell: their child octants. Old: (t6, v0, t7) =
    // (old >> (level diff - 1)) - 2 * cell; new: (s4, s5, s6) likewise.
    c.cycles += 28;
    let (cell, old, new) = (r32(c, S0), r32(c, T3), r32(c, FP));
    c.g[A3][0] = sx(r32(c, T4).wrapping_sub(r32(c, V1)));
    c.g[V0][0] = c.lw(cell.wrapping_add(4));
    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
    c.g[V1][0] = c.lw(old.wrapping_add(4));
    c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
    c.g[T5][0] = oc230_sll(c, V0, 1);
    c.g[A0][0] = c.lw(cell.wrapping_add(8));
    c.g[V1][0] = oc230_sra(c, V1, A2);
    c.g[V0][0] = c.lw(old.wrapping_add(0xc));
    c.g[T6][0] = sx(r32(c, V1).wrapping_sub(r32(c, T5)));
    c.g[T2][0] = oc230_sll(c, A0, 1);
    c.g[V1][0] = c.lw(cell.wrapping_add(0xc));
    c.g[V0][0] = oc230_sra(c, V0, A2);
    c.g[T0][0] = c.lw(old.wrapping_add(8));
    c.g[T1][0] = oc230_sll(c, V1, 1);
    c.g[A1][0] = c.lw(new.wrapping_add(4));
    c.g[A0][0] = c.lw(new.wrapping_add(8));
    c.g[T7][0] = sx(r32(c, V0).wrapping_sub(r32(c, T1)));
    c.g[V1][0] = c.lw(new.wrapping_add(0xc));
    c.g[T0][0] = oc230_sra(c, T0, A2);
    c.g[A1][0] = oc230_sra(c, A1, A3);
    c.g[A0][0] = oc230_sra(c, A0, A3);
    c.g[V1][0] = oc230_sra(c, V1, A3);
    c.g[V0][0] = sx(r32(c, T0).wrapping_sub(r32(c, T2)));
    c.g[S4][0] = sx(r32(c, A1).wrapping_sub(r32(c, T5)));
    c.g[S5][0] = sx(r32(c, A0).wrapping_sub(r32(c, T2)));
    c.g[S6][0] = sx(r32(c, V1).wrapping_sub(r32(c, T1)));
    let same_child = if c.g[T6][0] != c.g[S4][0] {
        c.cycles += 1;
        false
    } else {
        c.cycles += 2;
        if c.g[V0][0] != c.g[S5][0] {
            false
        } else {
            c.cycles += 2;
            c.g[T7][0] == c.g[S6][0]
        }
    };
    if same_child {
        // Recurse into the child with the child key (sp+0).
        c.cycles += 23;
        let sp = sp(c);
        c.g[V0][0] = oc230_sll(c, S4, 4);
        c.g[V1][0] = oc230_sll(c, S5, 3);
        c.g[V0][0] = oc230_add(c, S3, V0);
        c.g[A3][0] = oc230_add(c, T1, S6);
        c.g[A0][0] = sx(r32(c, T4).wrapping_sub(1));
        c.g[A1][0] = oc230_add(c, T5, S4);
        c.g[A2][0] = oc230_add(c, T2, S5);
        c.g[V0][0] = oc230_add(c, V0, V1);
        let v = c.g[A1][0];
        c.sw(sp.wrapping_add(4), v);
        c.g[V1][0] = oc230_sll(c, S6, 2);
        let v = c.g[A2][0];
        c.sw(sp.wrapping_add(8), v);
        c.g[V0][0] = oc230_add(c, V0, V1);
        let v = c.g[A3][0];
        c.sw(sp.wrapping_add(0xc), v);
        c.g[A1][0] = c.g[S7][0];
        let v = c.g[A0][0];
        c.sw(sp, v);
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[T3][0];
        c.g[T0][0] = c.g[FP][0];
        c.g[A0][0] = c.lw(r32(c, V0));
        c.g[T1][0] = c.g[SP][0];
        callee::MOVE.call(c, 0x0032_89c0)?;
        frame.restore(c, S0, 0xc0);
        oc230_shm_exit(c, frame);
        return Ok(());
    }

    // Different children: remove from the old child (key at sp+0, its
    // hash slot in s1).
    c.g[A3][0] = c.lw(cell.wrapping_add(8));
    c.cycles += 30;
    let sp = sp(c);
    c.g[A2][0] = oc230_sll(c, T6, 4);
    c.g[A0][0] = c.lw(cell.wrapping_add(4));
    c.g[A2][0] = oc230_add(c, S3, A2);
    c.g[A1][0] = c.lw(cell.wrapping_add(0xc));
    c.g[A3][0] = oc230_sll(c, A3, 1);
    c.g[V1][0] = c.lw(cell);
    c.g[A3][0] = oc230_add(c, A3, V0);
    c.g[V0][0] = oc230_sll(c, V0, 3);
    c.g[A0][0] = oc230_sll(c, A0, 1);
    c.g[A1][0] = oc230_sll(c, A1, 1);
    c.g[A2][0] = oc230_add(c, A2, V0);
    c.g[A0][0] = oc230_add(c, A0, T6);
    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
    c.g[A1][0] = oc230_add(c, A1, T7);
    let v = c.g[V1][0];
    c.sw(sp, v);
    c.g[V0][0] = oc230_sll(c, T7, 2);
    let v = c.g[A3][0];
    c.sw(sp.wrapping_add(8), v);
    let v = c.g[A1][0];
    c.sw(sp.wrapping_add(0xc), v);
    c.g[S1][0] = oc230_add(c, A2, V0);
    let v = c.g[A0][0];
    c.sw(sp.wrapping_add(4), v);
    c.g[A3][0] = c.g[T3][0];
    c.g[A1][0] = c.g[S7][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = c.lw(r32(c, S1));
    c.g[T0][0] = c.g[SP][0];
    callee::REMOVE.call(c, 0x0032_8a34)?;
    oc230_shm_after_remove(c, frame, Oc230Scan::Words)
}

/// After the remove from the old child: is its hash (`*s1`, eleven words)
/// empty? (Also stores the new child's offsets at sp+0x20/0x24/0x28 and the
/// key pointer sp+0x10 at sp+0x2c.) Then free it if so and insert into the
/// new child.
pub(crate) fn oc230_shm_after_remove(
    c: &mut Cpu,
    frame: Frame,
    mut probe: Oc230Scan,
) -> crate::call::Flow {
    use oc230_shm_callee as callee;
    let sp = sp(c);
    // The new child's offsets (x * 16, y * 8, z * 4) and the key pointer.
    let offsets = |c: &mut Cpu, reg: usize| {
        c.g[A3][0] = oc230_sll(c, S5, 3);
        let v = c.g[reg][0];
        c.sw(sp.wrapping_add(0x20), v);
        let v = c.g[A3][0];
        c.sw(sp.wrapping_add(0x24), v);
        c.g[reg][0] = oc230_sll(c, S6, 2);
        c.g[A3][0] = sx(sp.wrapping_add(0x10));
        let v = c.g[reg][0];
        c.sw(sp.wrapping_add(0x28), v);
        let v = c.g[A3][0];
        c.sw(sp.wrapping_add(0x2c), v);
    };
    loop {
        match probe {
            Oc230Scan::Words => {
                c.g[V1][0] = c.lw(r32(c, S1));
                let words = r32(c, V1);
                let mut set = false;
                for k in 0..7u32 {
                    if k > 0 {
                        c.cycles += 3;
                    }
                    c.g[V0][0] = c.lw(words.wrapping_add(4 * k));
                    if c.g[V0][0] != 0 {
                        set = true;
                        break;
                    }
                }
                if !set {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(words.wrapping_add(0x1c));
                    let clear = c.g[V0][0] == 0;
                    c.g[V0][0] = oc230_sll(c, S4, 4);
                    if clear {
                        // The last three words in a loop.
                        c.cycles += 10;
                        offsets(c, V0);
                        c.g[V1][0] = sx(words.wrapping_add(0x20));
                        c.g[A0][0] = 0;
                        probe = Oc230Scan::Tail;
                        continue;
                    }
                    c.cycles += 1;
                }
                c.g[V1][0] = oc230_sll(c, S4, 4);
                c.cycles += 9;
                c.g[V0][0] = 0;
                offsets(c, V1);
                break;
            }
            Oc230Scan::NonEmpty => {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break;
            }
            Oc230Scan::Tail => {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, V1));
                let set = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                if set {
                    if crate::events::back_edge_unwinds(c, SHM_NONEMPTY) {
                        return Ok(());
                    }
                    probe = Oc230Scan::NonEmpty;
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                if more {
                    if crate::events::back_edge_unwinds(c, SHM_TAIL) {
                        return Ok(());
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
                break;
            }
        }
    }

    // Free the old child's hash when empty.
    c.cycles += 2;
    let empty = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(sp.wrapping_add(0x20));
    if empty {
        c.cycles += 4;
        c.g[A0][0] = c.lw(r32(c, S1));
        callee::DELETE.call(c, 0x0032_8b1c)?;
        c.sw(r32(c, S1), 0);
        c.g[V0][0] = c.lw(sp.wrapping_add(0x20));
    }

    // The new child's key (sp+0x10) and hash slot (s1).
    c.cycles += 23;
    let cell = r32(c, S0);
    c.g[V1][0] = c.lw(sp.wrapping_add(0x24));
    c.g[A2][0] = oc230_add(c, S3, V0);
    c.g[A1][0] = c.lw(cell.wrapping_add(0xc));
    c.g[A0][0] = c.lw(cell.wrapping_add(4));
    c.g[A2][0] = oc230_add(c, A2, V1);
    c.g[V0][0] = c.lw(cell.wrapping_add(8));
    c.g[A1][0] = oc230_sll(c, A1, 1);
    c.g[V1][0] = c.lw(cell);
    c.g[A0][0] = oc230_sll(c, A0, 1);
    c.g[V0][0] = oc230_sll(c, V0, 1);
    c.g[A0][0] = oc230_add(c, A0, S4);
    c.g[V0][0] = oc230_add(c, V0, S5);
    c.g[A1][0] = oc230_add(c, A1, S6);
    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
    c.g[A3][0] = c.lw(sp.wrapping_add(0x28));
    let v = c.g[V1][0];
    c.sw(sp.wrapping_add(0x10), v);
    let v = c.g[A0][0];
    c.sw(sp.wrapping_add(0x14), v);
    c.g[S1][0] = oc230_add(c, A2, A3);
    let v = c.g[V0][0];
    c.sw(sp.wrapping_add(0x18), v);
    let v = c.g[A1][0];
    c.sw(sp.wrapping_add(0x1c), v);
    c.g[V0][0] = c.lw(r32(c, S1));
    if c.g[V0][0] != 0 {
        c.g[A0][0] = c.lw(r32(c, S1));
        return oc230_shm_insert(c, frame);
    }
    // No hash there yet: allocate one (0x2c bytes), clear it.
    c.cycles += 10;
    c.g[A1][0] = sx(0x004a_3fc0);
    c.g[A0][0] = 0x2c;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    callee::ALLOC_NAMED.call(c, 0x0032_8b98)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = u64::MAX;
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x20));
    c.g[V1][0] = 2;
    oc230_shm_clear_insert(c, frame)
}

/// The new hash (s0): words 8..10 zeroed in a loop (v0 pointer, v1 count
/// down to a0 = -1), `memset(h, 0, 0x20)`, stored in the slot (s1); then
/// the insert.
pub(crate) fn oc230_shm_clear_insert(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    loop {
        c.cycles += 7;
        c.sw(r32(c, V0), 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = c.g[V1][0] != c.g[A0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
        if !more {
            break;
        }
        if crate::events::back_edge_unwinds(c, SHM_CLEAR) {
            return Ok(());
        }
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x20;
    call_ptr(c, oc230_shm_callee::MEMSET, 0x0032_8bd4)?;
    let v = c.g[S0][0];
    c.sw(r32(c, S1), v);
    c.g[A0][0] = c.lw(r32(c, S1));
    oc230_shm_insert(c, frame)
}

/// `SpatialHash_InsertAtKey(*slot, key, e, new, child key)`, epilogue.
pub(crate) fn oc230_shm_insert(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 5;
    c.g[A1][0] = c.g[S7][0];
    c.g[T0][0] = c.lw(sp(c).wrapping_add(0x2c));
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[FP][0];
    oc230_shm_callee::INSERT_AT_KEY.call(c, 0x0032_8bf0)?;
    c.cycles += 1;
    frame.restore(c, S0, 0xc0);
    oc230_shm_exit(c, frame);
    Ok(())
}

pub(crate) mod spatial_insert {

    pub const NODE_LISTS: u32 = 0x20;

    pub const NODE_SIZE: u64 = 0x2c;

    /// Item: next (+0), prev (+4).
    pub const ITEM_PREV: u32 = 0x4;

    /// `EA_MemAllocNamed` name (0x4a3fc0) and flags for a new node.
    pub const NODE_NAME: u32 = 0x004a_3fc0;

    pub const NODE_FLAGS: u64 = 0x2000_0000;

    /// Loop heads (event checks).
    pub const DESCEND: u32 = 0x0032_86c0;

    pub const CLEAR: u32 = 0x0032_8778;
}

/// `void SpatialHash_InsertAtKey(Node* node, int list, Item* item, Cell*
/// target, Cell* at)` (0x328660): walk down the octree from `node` (whose
/// cell is `*at`) to the cell `target`, one level at a time, creating
/// missing child nodes (`EA_MemAllocNamed(0x2c, name, 0x20000000, 0)`,
/// lists cleared, children `memset` to 0), then push `item` onto the
/// target node's list `list`. Frame 0x80 (s0-s5; the walked cell at sp+0).
pub fn spatial_hash_insert_at_key(c: &mut Cpu) {
    use spatial_insert::*;
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x70);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        // The walked cell: a copy of *at at sp+0.
        let at = r32(c, T0);
        let cell = sp(c);
        c.ldl(V0, at.wrapping_add(7));
        c.ldr(V0, at);
        c.ldl(V1, at.wrapping_add(0xf));
        c.ldr(V1, at.wrapping_add(8));
        c.sdl(V0, cell.wrapping_add(7));
        c.sdr(V0, cell);
        c.sdl(V1, cell.wrapping_add(0xf));
        c.sdr(V1, cell.wrapping_add(8));
        let target = r32(c, S2);
        c.g[V1][0] = Addr(target).lw(c);
        c.g[V0][0] = Addr(cell).lw(c);
        c.g[T2][0] = c.g[A0][0];
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 1;
            c.g[S5][0] = 0x004a_0000;
            loop {
                spatial_hash_insert_descend(c)?;
                c.cycles += 3;
                let target_level = Addr(r32(c, S2)).lw(c);
                c.g[V0][0] = target_level;
                c.g[T2][0] = Addr(r32(c, S1)).lw(c);
                if c.g[V1][0] == target_level {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, DESCEND);
                }
            }
        }
        // Push the item onto the node's list.
        c.cycles += 6;
        let off = sx(r32(c, S4) << 2);
        c.g[V0][0] = off;
        let head_at = (off as u32)
            .wrapping_add(r32(c, T2))
            .wrapping_add(NODE_LISTS);
        c.g[V1][0] = sx(head_at);
        let head = Addr(head_at).lw(c);
        c.g[V0][0] = head;
        let item = r32(c, S3);
        if head != 0 {
            Addr((head as u32).wrapping_add(ITEM_PREV)).sw(c, item as u64);
        }
        c.cycles += 13;
        let head = Addr(head_at).lw(c);
        c.g[V0][0] = head;
        Addr(item.wrapping_add(ITEM_PREV)).sw(c, 0);
        Addr(item).sw(c, head);
        Addr(head_at).sw(c, item as u64);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// One level down (block 0x3286c0 up to 0x3287ac): step the walked cell
/// one level toward the target, select the child (s1 = its slot), create
/// it if missing. Leaves v1 = the new level.
pub(crate) fn spatial_hash_insert_descend(c: &mut Cpu) -> crate::call::Flow {
    use spatial_insert::*;
    c.cycles += 36;
    let cell = sp(c);
    let target = r32(c, S2);
    let level = Addr(cell).lw(c) as u32;
    let target_level = Addr(target).lw(c) as u32;
    c.g[T1][0] = sx(level.wrapping_sub(1));
    let sh = level.wrapping_sub(target_level).wrapping_sub(1);
    c.g[V1][0] = sx(sh);
    let tx = Addr(target.wrapping_add(4)).lw(c) as u32;
    let ty = Addr(target.wrapping_add(8)).lw(c) as u32;
    let tz = Addr(target.wrapping_add(0xc)).lw(c) as u32;
    let cx2 = (Addr(cell.wrapping_add(4)).lw(c) as u32) << 1;
    let cy2 = (Addr(cell.wrapping_add(8)).lw(c) as u32) << 1;
    let cz2 = (Addr(cell.wrapping_add(0xc)).lw(c) as u32) << 1;
    let shr = |v: u32| ((v as i32) >> (sh & 31)) as u32;
    let dx = shr(tx).wrapping_sub(cx2);
    let dy = shr(ty).wrapping_sub(cy2);
    let dz = shr(tz).wrapping_sub(cz2);
    let nx = cx2.wrapping_add(dx);
    let ny = cy2.wrapping_add(dy);
    let nz = cz2.wrapping_add(dz);
    c.g[A1][0] = sx(dx);
    c.g[T0][0] = sx(nx);
    c.g[A2][0] = sx(ny);
    c.g[A3][0] = sx(nz);
    let row = (dx << 4).wrapping_add(r32(c, T2)).wrapping_add(dy << 3);
    Addr(cell).sw(c, sx(level.wrapping_sub(1)));
    Addr(cell.wrapping_add(4)).sw(c, sx(nx));
    let dz4 = dz << 2;
    c.g[A0][0] = sx(dz4);
    Addr(cell.wrapping_add(8)).sw(c, sx(ny));
    let slot = row.wrapping_add(dz4);
    c.g[S1][0] = sx(slot);
    Addr(cell.wrapping_add(0xc)).sw(c, sx(nz));
    let child = Addr(slot).lw(c);
    c.g[V0][0] = child;
    c.g[V1][0] = Addr(cell).lw(c);
    if child == 0 {
        // A new node: lists cleared, children zeroed.
        c.cycles += 10;
        c.g[A0][0] = NODE_SIZE;
        c.g[A1][0] = sx(r32(c, S5).wrapping_add(NODE_NAME & 0xffff));
        c.g[A2][0] = NODE_FLAGS;
        c.g[A3][0] = 0;
        Callee {
            addr: 0x0031_7d70,
            f: ee::s10::f_00317d70,
        }
        .call(c, 0x0032_8764)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(NODE_LISTS));
        c.g[V1][0] = 2;
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, 0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, CLEAR);
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x20;
        // memset(node, 0, 0x20)
        call_ptr(c, 0x003e_6448, 0x0032_87a4)?;
        let node = c.g[S0][0];
        Addr(r32(c, S1)).sw(c, node);
        c.g[V1][0] = Addr(sp(c)).lw(c);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// AIFwdDiffCache_BuildEntry (0x327c00) / AIFwdDiffCache_ResetEntries (0x327c68)

/// The VU0 register block the forward-difference microprogram works on
/// (`gp+0x2930`).
pub(crate) const AFD_VU_REGS: u32 = 0x2930;

/// Offset of the source curve's coefficient block in the source node.
pub(crate) const AFD_SRC_COEFFS: u32 = 0x40;

/// Offset of the output samples in the cache entry.
pub(crate) const AFD_ENTRY_SAMPLES: u32 = 0x10;

/// The two blocks of sample rows ResetEntries fills, their row counts
/// (minus one) and the gp-relative "far away" fill values.
pub(crate) const AFD_ROWS_A: u32 = 0x650;

pub(crate) const AFD_ROWS_A_LAST: u64 = 0x50;

pub(crate) const AFD_FILL_A: u32 = 0x2e04;

pub(crate) const AFD_ROWS_B: u32 = 0x1a90;

pub(crate) const AFD_ROWS_B_LAST: u64 = 0x8;

pub(crate) const AFD_FILL_B: u32 = 0x2e00;

pub(crate) const AFD_VU0_LOAD_REGS_AND_CALL: u32 = 0x0039_1418;

pub(crate) const AFD_VU0_READ_VI1: u32 = 0x0039_14f8;

pub(crate) const AFD_VU0_COPY_OUTPUT_MEM: u32 = 0x0039_1480;

pub(crate) const AFD_RESET_ENTRIES: Callee = Callee {
    addr: 0x0032_7c68,
    f: ee::s10::AIFwdDiffCache_ResetEntries,
};

/// `void AIFwdDiffCache_BuildEntry(AIFwdDiffCache* self, Node* src,
/// Entry* entry)` (0x327c00): loads the source curve's 16 coefficient
/// quadwords into VU0 and runs the forward-difference microprogram, resets
/// the entry's sample rows to "far away", reads VI1 and copies the VU0
/// output (10 x 0xa0 bytes) into the entry's samples.
pub fn ai_fwd_diff_cache_build_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        let src = reg(c, A1);
        set_reg(c, A1, addu(src, AFD_SRC_COEFFS as u64));
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = reg(c, A0);
        set_reg(c, S0, this);
        frame.save(c, S2, 0x10);
        let regs = sx((c.g[GP][0] as u32).wrapping_add(AFD_VU_REGS));
        set_reg(c, S1, regs);
        let entry = reg(c, A2);
        set_reg(c, S2, entry);
        frame.save_ra(c, 0);
        set_reg(c, A0, regs);
        call_ptr(c, AFD_VU0_LOAD_REGS_AND_CALL, 0x0032_7c2c)?;

        let entry = reg(c, S2);
        set_reg(c, A1, entry);
        let this = reg(c, S0);
        set_reg(c, A0, this);
        AFD_RESET_ENTRIES.call(c, 0x0032_7c38)?;

        let regs = reg(c, S1);
        set_reg(c, A0, regs);
        call_ptr(c, AFD_VU0_READ_VI1, 0x0032_7c40)?;

        let regs = reg(c, S1);
        set_reg(c, A0, regs);
        let entry = reg(c, S2);
        set_reg(c, A1, addu(entry, AFD_ENTRY_SAMPLES as u64));
        call_ptr(c, AFD_VU0_COPY_OUTPUT_MEM, 0x0032_7c4c)?;

        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Fill rows `x, +0x10, +0x20` of `last + 1` 0x40-byte sample rows starting
/// at `entry + rows` with f0 (the loop head is at `head`).
pub(crate) fn afd_fill_rows(c: &mut Cpu, entry: u32, rows: u32, last: u64, head: u32) {
    c.g[V1][0] = sx(entry.wrapping_add(rows));
    c.g[A0][0] = last;
    loop {
        c.cycles += 7;
        let row = r32(c, V1);
        c.swc1(0, row);
        c.g[V0][0] = c.g[A0][0];
        c.swc1(0, row.wrapping_add(0x10));
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
        c.swc1(0, row.wrapping_add(0x20));
        let more = (c.g[V0][0] as i64) > 0;
        c.g[V1][0] = sx(row.wrapping_add(0x40));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void AIFwdDiffCache_ResetEntries(AIFwdDiffCache* self, Entry* entry)`
/// (0x327c68): marks every sample of the entry's two row blocks (81 rows
/// at +0x650, 9 rows at +0x1a90; three floats per row) as "far away" with
/// the gp-relative fill values, so stale samples never win a distance test.
pub fn ai_fwd_diff_cache_reset_entries(c: &mut Cpu) {
    c.cycles += 4;
    let gp = c.g[GP][0] as u32;
    let entry = r32(c, A1);
    c.lwc1(0, gp.wrapping_sub(AFD_FILL_A));
    afd_fill_rows(c, entry, AFD_ROWS_A, AFD_ROWS_A_LAST, 0x0032_7c78);
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    let entry = r32(c, A1);
    c.lwc1(0, gp.wrapping_sub(AFD_FILL_B));
    afd_fill_rows(c, entry, AFD_ROWS_B, AFD_ROWS_B_LAST, 0x0032_7ca0);
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP as TEST_SP;
    pub(crate) use crate::game::ai::ai::tests::generic;
    pub(crate) use crate::game::ai::ai::tests::globals_8 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_8 as WORLD;
    pub(crate) use crate::game::intersect::worldsphtree::tests::f;
    pub(crate) use crate::game::intersect::worldsphtree::tests::uf;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;

    #[test]
    fn ai_fwd_diff_cache_touch_matches() {
        check(
            ssx3_ee::s10::f_00327ac0,
            ai_fwd_diff_cache_touch,
            200,
            &|c, seed| {
                let mut r = Rng(0x100e ^ seed as u64);
                scramble(c, &mut r);
                let node = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = node as u64;
                let head = DATA + 0x200;
                let next = DATA + 0x300;
                match seed % 3 {
                    0 => c.bus.write32(DATA + 4, node),
                    1 => {
                        c.bus.write32(DATA + 4, head);
                        c.bus.write32(head + 8, node);
                    }
                    _ => {
                        c.bus.write32(DATA + 4, head);
                        c.bus.write32(head + 8, next);
                        c.bus.write32(node + 4, DATA + 0x400);
                        c.bus.write32(node + 8, DATA + 0x500);
                        c.bus.write32(next + 4, r.next_u32());
                    }
                }
            },
        );
    }

    // --------------------------------------------------- assorted leaves

    #[test]
    fn ai_fwd_diff_cache_hash_remove_matches() {
        check(
            ee::s10::f_00327b30,
            ai_fwd_diff_cache_hash_remove,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x327b30 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                let node = DATA + 0x100;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = node as u64;
                let count = (seed % 4) + 1;
                c.bus.write32(this + 0xc, count);
                let buckets = DATA + 0x400;
                c.bus.write32(this + 8, buckets);
                let key = if seed % 6 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(node, key);
                // A short chain in the selected bucket; one node matches `key`
                // when seed % 3 != 0 so the removal path runs.
                let bucket = buckets + (key % count) * 4;
                let n = (seed % 3) + 1;
                let mut cur = bucket;
                for i in 0..n {
                    let e = DATA + 0x800 + i * 0x10;
                    c.bus.write32(cur, e);
                    let k = if seed % 3 != 0 && i == n - 1 {
                        key
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(e, k);
                    cur = e + 0xc;
                }
                c.bus.write32(cur, 0);
            },
        );
    }

    #[test]
    fn ai_fwd_diff_cache_hash_insert_matches() {
        check(
            ee::s10::f_00327bb0,
            ai_fwd_diff_cache_hash_insert,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0032_7bb0 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                let buckets = DATA + 0x200;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let count = (seed % 4) + 1;
                c.bus.write32(this + 0xc, count);
                c.bus.write32(this + 8, buckets);
                c.bus.write32(DATA + 0x100, r.next_u32());
                // Build a short descending chain in the selected bucket.
                let bucket = buckets + (c.bus.read32(DATA + 0x100) % count) * 4;
                let n = seed % 3;
                let mut cur = bucket;
                for _ in 0..n {
                    let node = DATA + 0x400 + (r.next_u32() & 0xf0);
                    c.bus.write32(cur, node);
                    c.bus.write32(node, r.next_u32());
                    cur = node + 0xc;
                }
                c.bus.write32(cur, 0);
            },
        );
    }

    #[test]
    fn spatial_key_from_aabb_matches() {
        check(
            ee::s10::f_00328f28__generated,
            spatial_key_from_aabb,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x0032_8f28 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = SK_ENTRY;
                let scale = [1.0f32, 0.5, 2.0, 0.01][(seed % 4) as usize];
                c.bus.write32(GAME_GP.wrapping_sub(0x2df8), f(scale));
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x2df4), f(uf(&mut r, 0.0, 0.5)));
                let aabb = DATA + 0x100;
                for k in 0..3 {
                    let lo = uf(&mut r, -20000.0, 20000.0);
                    let ext = match (seed / 4 + k) % 4 {
                        0 => 0.0,
                        1 => uf(&mut r, 0.0, 10.0),
                        2 => uf(&mut r, 0.0, 3000.0),
                        _ => uf(&mut r, 0.0, 40000.0),
                    };
                    c.bus.write32(aabb + 4 * k, f(lo));
                    c.bus.write32(aabb + 0x10 + 4 * k, f(lo + ext));
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(aabb);
            },
        );
    }

    #[test]
    fn spatial_key_classify_aabb_matches() {
        check(
            ee::s10::f_00328360__generated,
            spatial_key_classify_aabb,
            300,
            &|c, seed| {
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
                        if k < 2 {
                            5
                        } else {
                            0
                        }
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
            },
        );
    }

    /// The spatial hash test world: level-11 keys (tiny scale), roots at
    /// level 11 holding the coordinates the case needs.
    #[test]
    fn spatial_hash_update_matches() {
        check(
            ee::s10::f_003291e0__generated,
            spatial_hash_update,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3291, 0);
                let (hash, obj_node, old_box, new_box) =
                    (DATA, DATA + 0x3000, DATA + 0x100, DATA + 0x140);
                c.bus.write32(gp_at(-0x2df8), f(1.0e-6));
                c.bus.write32(gp_at(-0x2df4), f(0.0));
                let case = seed % 4;
                let cell = |r: &mut Rng| -> i32 { (r.next_u32() % 6) as i32 - 3 };
                let old: [i32; 3] = [cell(&mut r), cell(&mut r), cell(&mut r)];
                let mut new = old;
                match case {
                    0 => {}
                    1 => new[0] = if old[0] < 0 { 1 } else { -2 },
                    2 => {
                        new[1] = if old[1] == -1 {
                            -2
                        } else if old[1] == 0 {
                            1
                        } else {
                            old[1] + old[1].signum()
                        }
                    }
                    _ => {
                        new[2] = if old[2] == -1 {
                            -2
                        } else if old[2] == 0 {
                            1
                        } else {
                            old[2] + old[2].signum()
                        }
                    }
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
                    let arr = if empty {
                        0x200
                    } else {
                        DATA + 0x1000 + 0x100 * i as u32
                    };
                    c.bus.write32(root + 0x10, arr);
                    if !empty && seed % 2 == 0 {
                        let w = r.next_u32() % 8;
                        c.bus.write32(arr + 4 * w, 1 + r.next_u32() % 100);
                    }
                }
                let empty = case == 1 && seed % 8 == 1;
                c.bus.write32(
                    obj_node,
                    if empty || seed % 2 == 0 {
                        0
                    } else {
                        DATA + 0x3100
                    },
                );
                c.bus.write32(obj_node + 4, 0);
                c.g[A0][0] = sx(hash);
                // (List heads 0..2 lie inside the scanned words.)
                c.g[A1][0] = (r.next_u32() % 3) as u64;
                c.g[A2][0] = sx(obj_node);
                c.g[A3][0] = sx(new_box);
                c.g[T0][0] = sx(old_box);
            },
        );
    }

    #[test]
    fn roller_modifier_copy_matrix_block_matches() {
        diff(
            ee::s10::f_00327cc8__generated,
            roller_modifier_copy_matrix_block,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x32_7cc8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(if seed % 2 == 0 {
                    DATA + 0x2000
                } else {
                    DATA + 0x2004
                });
                for i in 0..0x40 {
                    w(c, DATA + 0x2000 + 4 * i, r.next_u32());
                }
                for i in 0..0x40 {
                    w(c, DATA + 0x2200 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// A deterministic float for fixture fields.
    pub(crate) fn edge(seed: u32, k: u32) -> u32 {
        let mut r = Rng(0x328030 ^ seed as u64 ^ (k as u64) << 32);
        r.f32(10.0)
    }

    #[test]
    fn sphere_tree_collide_triangle2_sided_matches() {
        same(
            ssx3_ee::s10::f_00328030__generated,
            sphere_tree_collide_triangle2_sided,
            200,
            &|c, seed| {
                generic(c, seed, 0x40);
                let mut r = crate::difftest::Rng((seed as u64) ^ 0x9e37_79b9);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[T0][0] = (DATA + 0x900) as u64;
                put_vec(
                    c,
                    DATA + 0x180,
                    [edge(seed, 1), edge(seed, 2), edge(seed, 3), 0],
                );
            },
        );
    }
    use super::*;
    use super::*;

    use crate::difftest::fresh;

    /// `AIFwdDiffCache_Get`: a cache of three nodes on an LRU ring, hashed
    /// into sorted bucket chains (some unused, key -1); the source key is a
    /// cached one (hit: Touch) or a new one (miss: rehash, rebuild through
    /// the VU0 helpers, whose micro program is a stub that ends at once).
    #[test]
    fn ai_fwd_diff_cache_get_matches() {
        use crate::difftest::elf_image;
        check_lohi(
            ee::s10::f_003279d0__generated,
            ai_fwd_diff_cache_get,
            200,
            &|c, seed| {
                let mut r = Rng(0x3279_d000 ^ seed as u64);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x0032_79d0;
                // VU0 micro program at 0: NOP / NOP[E], NOP / NOP.
                put_words(
                    c,
                    0x1100_0000,
                    &[0x8000_033c, 0x4000_02ff, 0x8000_033c, 0x0000_02ff],
                );
                let this = DATA;
                let src = DATA + 0x100;
                let out = DATA + 0x300;
                let buckets = DATA + 0x400;
                let nodes = [DATA + 0x1000, DATA + 0x3000, DATA + 0x5000];
                let count = 1 + r.next_u32() % 4;
                c.bus.write32(this + FDC_LRU, nodes[(seed % 3) as usize]);
                c.bus.write32(this + FDC_BUCKETS, buckets);
                c.bus.write32(this + FDC_COUNT, count);
                fill(c, &mut r, src, 0x60);
                // Distinct keys (some unused), the LRU ring 0 -> 1 -> 2 -> 0.
                let mut keys = [0u32; 3];
                for i in 0..3 {
                    keys[i] = if r.next_u32() % 4 == 0 {
                        u32::MAX
                    } else {
                        (i as u32) * 7 + r.next_u32() % 7
                    };
                    c.bus.write32(nodes[i] + FDN_KEY, keys[i]);
                    c.bus.write32(nodes[i] + FDN_LRU_NEXT, nodes[(i + 1) % 3]);
                    c.bus.write32(nodes[i] + 0x8, nodes[(i + 2) % 3]);
                }
                // Bucket chains, ascending by key.
                for b in 0..count {
                    let mut chain: Vec<usize> = (0..3)
                        .filter(|&i| keys[i] != u32::MAX && keys[i] % count == b)
                        .collect();
                    chain.sort_by_key(|&i| keys[i]);
                    let mut link = buckets + 4 * b;
                    for &i in &chain {
                        c.bus.write32(link, nodes[i]);
                        link = nodes[i] + FDN_HASH_NEXT;
                    }
                    c.bus.write32(link, 0);
                }
                let used: Vec<u32> = keys.iter().copied().filter(|&k| k != u32::MAX).collect();
                let key = if seed % 2 == 0 && !used.is_empty() {
                    used[(r.next_u32() as usize) % used.len()]
                } else {
                    r.next_u32() % 24
                };
                c.bus.write32(src + FD_SRC_KEY, key);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(src);
                c.g[A2][0] = sx(out);
            },
        );
    }

    // ---- Collide_SphereVsBoxEdges (0x329590) ----

    /// Boxes around the origin, centres inside and outside them on every
    /// axis, radii 0..5 (some exactly on a face), the axis unit vectors
    /// in .bss and the 1e30 `.lit4` limits.
    pub(crate) fn collide_sphere_vs_box_edges_setup(c: &mut Cpu, seed: u32) {
        const B: u32 = 0x00b0_0000;
        let mut r = Rng(0x0032_9590 ^ (seed as u64) << 3);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        put_words(c, GAME_GP - 0x2df0, &[0x7149_f2ca, 0x7149_f2ca]);
        let one = 1.0f32.to_bits();
        put_vec(c, 0x004f_f140, [one, 0, 0, 0]);
        put_vec(c, 0x004f_f150, [0, one, 0, 0]);
        put_vec(c, 0x004f_f160, [0, 0, one, 0]);
        let coord = |r: &mut Rng, lo: f32, hi: f32| {
            (lo + (r.next_u32() % 1000) as f32 * (hi - lo) / 1000.0).to_bits()
        };
        for i in 0..3 {
            c.bus.write32(B + 4 * i, coord(&mut r, -10.0, 0.0));
            c.bus.write32(B + 0x10 + 4 * i, coord(&mut r, 0.0, 10.0));
            // Mostly inside the box so the face tests are reached.
            let centre = if r.next_u32() % 4 == 0 {
                coord(&mut r, -15.0, 15.0)
            } else {
                coord(&mut r, -9.0, 9.0)
            };
            c.bus.write32(B + 0x20 + 4 * i, centre);
        }
        c.bus.write32(B + 0x2c, one);
        fill(c, &mut r, B + 0x30, 0x10);
        c.g[A0][0] = B as u64;
        c.g[A1][0] = (B + 0x10) as u64;
        c.g[A2][0] = (B + 0x20) as u64;
        c.g[A3][0] = (B + 0x30) as u64;
        c.g[T0][0] = (B + 0x40) as u64;
        c.g[T1][0] = (B + 0x50) as u64;
        c.f[12] = if seed % 10 == 0 {
            0
        } else {
            coord(&mut r, 0.0, 5.0)
        };
        c.pc = 0x0032_9590;
    }

    #[test]
    fn collide_sphere_vs_box_edges_matches() {
        check(
            ee::s10::f_00329590__generated,
            collide_sphere_vs_box_edges,
            300,
            &collide_sphere_vs_box_edges_setup,
        );
    }
    use super::*;

    /// SpatialHash_Move on a small octree: a cell at level 2 or 3, the
    /// entry linked (with or without neighbours) under the old key, the new
    /// key at the cell's level, below it in the same child (recursion) or in
    /// another child (old child freed when it empties, new child allocated
    /// when absent). Child hashes come from the real heap (`alloc_ready`);
    /// the real Remove/InsertAtKey run.
    #[test]
    fn spatial_hash_move_matches() {
        use crate::game::be::belibrary::tests::{alloc_ready, load};
        check(
            ee::s10::f_00328808__generated,
            spatial_hash_move,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_8808 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                alloc_ready(c);
                let alloc = |c: &mut Cpu| -> u32 {
                    let p = load(
                        c,
                        0x0031_7d70,
                        ee::s10::EA_MemAllocNamed,
                        &[0x2c, 0x004a_3fc0, 0x2000_0000, 0],
                    ) as u32;
                    for i in 0..11 {
                        c.bus.write32(p + 4 * i, 0);
                    }
                    p
                };
                let level = 2 + (seed / 4 % 2) as i32;
                let cell: [i32; 3] = [0, 1, 2].map(|_| (r.next_u32() % 5) as i32 - 2);
                // A key at level l under the cell (octant bits random).
                let key_at = |r: &mut Rng, l: i32| -> [i32; 3] {
                    let n = level - l;
                    cell.map(|v| (v << n) | (r.next_u32() & ((1 << n) - 1)) as i32)
                };
                let chain = |c: &mut Cpu, h: u32, l: i32, k: &[i32; 3]| -> u32 {
                    let mut node = h;
                    for j in (l..level).rev() {
                        let o = k.map(|v| ((v >> (j - l)) & 1) as u32);
                        let slot = node + o[0] * 16 + o[1] * 8 + o[2] * 4;
                        let mut child = c.bus.read32(slot);
                        if child == 0 {
                            child = alloc(c);
                            c.bus.write32(slot, child);
                        }
                        node = child;
                    }
                    node
                };
                let h = DATA + 0x2000;
                for i in 0..11 {
                    c.bus.write32(h + 4 * i, 0);
                }
                let key = r.next_u32() % 3;
                let (cell_k, old_k, new_k) = (DATA + 0x100, DATA + 0x110, DATA + 0x120);
                let deeper = |r: &mut Rng| level - 1 - (r.next_u32() % level as u32) as i32;
                let (old_l, new_l) = match seed % 4 {
                    0 => (
                        if r.next_u32() % 2 == 0 {
                            level
                        } else {
                            deeper(&mut r)
                        },
                        level,
                    ),
                    1 => (level, deeper(&mut r)),
                    _ => (deeper(&mut r), deeper(&mut r)),
                };
                let old = key_at(&mut r, old_l);
                let mut new = key_at(&mut r, new_l);
                if seed % 4 == 2 {
                    // Same top octant as the old key.
                    let (no, oo) = (level - 1 - new_l, level - 1 - old_l);
                    for a in 0..3 {
                        let bit = (old[a] >> oo) & 1;
                        new[a] = (new[a] & !(1 << no)) | (bit << no);
                    }
                }
                for (at, l, k) in [
                    (cell_k, level, cell),
                    (old_k, old_l, old),
                    (new_k, new_l, new),
                ] {
                    c.bus.write32(at, l as u32);
                    for a in 0..3 {
                        c.bus.write32(at + 4 + 4 * a as u32, k[a] as u32);
                    }
                }
                // The entry (and maybe neighbours) in the old key's bucket.
                let bottom = chain(c, h, old_l, &old);
                let bucket = bottom + 0x20 + 4 * key;
                let (e, prev, next) = (DATA + 0x200, DATA + 0x210, DATA + 0x220);
                let with_prev = r.next_u32() % 3 == 0;
                let with_next = r.next_u32() % 3 == 0;
                put_words(
                    c,
                    e,
                    &[
                        if with_next { next } else { 0 },
                        if with_prev { prev } else { 0 },
                    ],
                );
                put_words(c, prev, &[e, 0]);
                put_words(c, next, &[0, e]);
                c.bus.write32(bucket, if with_prev { prev } else { e });
                // Other live words in the old bottom hash, sometimes.
                if r.next_u32() % 3 == 0 {
                    let w = r.next_u32() % 11;
                    if bottom + 4 * w != bucket && c.bus.read32(bottom + 4 * w) == 0 {
                        c.bus.write32(bottom + 4 * w, DATA + 0x7000);
                    }
                }
                // The new key's chain already there, sometimes.
                if r.next_u32() % 2 == 0 && new_l < level {
                    chain(c, h, new_l, &new);
                }
                c.g[A0][0] = sx(h);
                c.g[A1][0] = key as u64;
                c.g[A2][0] = sx(e);
                c.g[A3][0] = sx(old_k);
                c.g[T0][0] = sx(new_k);
                c.g[T1][0] = sx(cell_k);
                c.pc = SHM_ENTRY;
            },
        );
    }
    use super::*;

    // -- 0x00328660 SpatialHash_InsertAtKey ------------------------------------

    /// `SpatialHash_InsertAtKey` against the generated body: the target cell
    /// 0-3 levels below the start cell (start cell copy aligned or not),
    /// child nodes present or created (the real `EA_MemAllocNamed` on a
    /// test heap, its mutex syscalls fakes that sometimes unwind; the real
    /// `memset`), lists empty or not.
    #[test]
    fn spatial_hash_insert_at_key_matches() {
        use super::spatial_insert::*;
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        use crate::game::bx::bxstring::tests::alloc_ready;
        const TARGET: u32 = DATA + 0x20_0000;
        const ROOT: u32 = DATA + 0x20_1000;
        const PRE: u32 = DATA + 0x20_2000;
        const OLD: u32 = DATA + 0x20_3000;
        const ITEM: u32 = DATA + 0x20_4000;
        check(
            ee::s10::f_00328660__generated,
            spatial_hash_insert_at_key,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_8660, 3);
                alloc_ready(c);
                c.pc = 0x0032_8660;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                let level = r.next_u32() % 3;
                let depth = r.next_u32() % 4;
                let at = TARGET + if r.next_u32() % 2 == 0 { 0x10 } else { 0x13 };
                w(c, TARGET, level);
                c.bus.write32(at - (at & 3), 0);
                for k in 0..3u32 {
                    let v = r.next_u32() % 64;
                    w(c, TARGET + 4 + 4 * k, v);
                    for (b, byte) in (v >> depth).to_le_bytes().into_iter().enumerate() {
                        c.bus.write8(at + 4 + 4 * k + b as u32, byte);
                    }
                }
                for (b, byte) in (level + depth).to_le_bytes().into_iter().enumerate() {
                    c.bus.write8(at + b as u32, byte);
                }
                // Root and pre-made nodes: some children present, lists empty or not.
                for n in 0..9u32 {
                    let node = if n == 0 { ROOT } else { PRE + 0x40 * (n - 1) };
                    for k in 0..8u32 {
                        let child = if n == 0 && r.next_u32() % 2 == 0 {
                            PRE + 0x40 * k
                        } else {
                            0
                        };
                        w(c, node + 4 * k, child);
                    }
                    for l in 0..3u32 {
                        let head = if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            OLD + 0x10 * (n * 3 + l)
                        };
                        w(c, node + NODE_LISTS + 4 * l, head);
                    }
                }
                c.g[A0][0] = sx(ROOT);
                c.g[A1][0] = (r.next_u32() % 3) as u64;
                c.g[A2][0] = sx(ITEM);
                c.g[A3][0] = sx(TARGET);
                c.g[T0][0] = sx(at);
            },
        );
    }
    use super::*;

    /// Stage the VU0 microprogram at address 0 as `NOP / NOP[E]` so
    /// Vu0_LoadRegsAndCall's `vcallmsr` (CMSAR0 = 0) returns at once.
    pub(crate) fn afd_stub_microprogram(c: &mut Cpu) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .hw
            .vu0
            .fallback = Some(ssx3_ps2::vu::reference::run_program);
        c.bus.write32(0x1100_0000, 0x8000_033c); // NOP (lower)
        c.bus.write32(0x1100_0004, 0x4000_02ff); // NOP, E (upper)
        c.bus.write32(0x1100_0008, 0x8000_033c); // NOP
        c.bus.write32(0x1100_000c, 0x0000_02ff); // NOP
    }

    /// AIFwdDiffCache_BuildEntry: real callees (the VU0 loader with a stub
    /// microprogram, ResetEntries, VI1 read, output copy) on random source
    /// coefficients, random VU0 output memory and random fill values.
    #[test]
    fn ai_fwd_diff_cache_build_entry_matches() {
        check(
            ee::s10::f_00327c00__generated,
            ai_fwd_diff_cache_build_entry,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_7c00, 0);
                c.pc = 0x0032_7c00;
                afd_stub_microprogram(c);
                let this = DATA + 0x20_0000;
                let src = DATA + 0x20_0100;
                let entry = DATA + 0x20_1000;
                fill(c, &mut r, src + AFD_SRC_COEFFS, 0x40);
                fill(c, &mut r, 0x1100_4200, 0x190);
                fill(c, &mut r, entry, 0x740);
                c.bus.write32(GAME_GP - AFD_FILL_A, r.next_u32());
                c.bus.write32(GAME_GP - AFD_FILL_B, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(src);
                c.g[A2][0] = sx(entry);
            },
        );
    }

    /// ResetEntries fixture: an entry at a random place, random rows and
    /// fill values.
    pub(crate) fn afd_reset_entries_setup(c: &mut Cpu, r: &mut Rng) {
        scramble(c, r);
        c.pc = 0x0032_7c68;
        c.g[GP][0] = GAME_GP as u64;
        let entry = DATA + 0x20_0000 + (r.next_u32() & 0xff0);
        fill(c, r, entry + AFD_ROWS_A, 0x80);
        c.bus.write32(GAME_GP - AFD_FILL_A, r.edge_f32());
        c.bus.write32(GAME_GP - AFD_FILL_B, r.edge_f32());
        c.g[A1][0] = sx(entry);
    }

    /// AIFwdDiffCache_ResetEntries: both row blocks filled.
    #[test]
    fn ai_fwd_diff_cache_reset_entries_matches() {
        check(
            ee::s10::f_00327c68__generated,
            ai_fwd_diff_cache_reset_entries,
            200,
            &|c, seed| {
                let mut r = Rng(0x32_7c68 ^ (seed as u64) << 16);
                afd_reset_entries_setup(c, &mut r);
            },
        );
    }

    /// AIFwdDiffCache_ResetEntries with events due at the loops' back edges.
    #[test]
    fn ai_fwd_diff_cache_reset_entries_events_match() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s10::f_00327c68__generated,
            ai_fwd_diff_cache_reset_entries,
            90,
            &|c, seed| {
                let mut r = Rng(0x32_7c69 ^ (seed as u64) << 16);
                afd_reset_entries_setup(c, &mut r);
                let ev = schedule(c, &mut r, seed, 0x0032_7c68);
                c.g[GP][0] = GAME_GP as u64;
                ev
            },
        );
    }
}
