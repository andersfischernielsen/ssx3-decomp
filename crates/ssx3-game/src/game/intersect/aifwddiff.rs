//! `src/intersect/aifwddiff.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
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
}
