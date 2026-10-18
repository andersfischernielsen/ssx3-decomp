//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for world collision queries (cells, overlap lists, rails, spatial keys), each with a differential test against the generated
//! body.
//!
//! - **Cells** (`Cell_GatherObjects`, `Cell_CollideInstance`,
//!   `Cell_CollideObject`) and **overlap lists**
//!   (`OverlapList_CollideVolume`): AABB rejection chains (six float
//!   compares) in front of the forward-difference cache
//!   (`AIFwdDiffCache_Get`/`_CollideEntry`) or the per-object collider
//!   (`Collide_VolumeObject`); hits are appended as 0x80-byte contact
//!   records.
//! - **Spatial keys** (`SpatialKey_FromAABB`): `{level, x, y, z}` of the
//!   smallest power-of-two grid cell (level >= 11) that holds a box.
//! - **World painter** (`WorldPainterQuery_UpdateAtPos`) and **world sound
//!   octree** (`cWorldSound_QueryTreeNode`) queries.
//!
//! The replacements keep their state in the guest registers the originals
//! use (callers and event handlers see the same values), follow the call
//! protocol of [`crate::call`] and the event rules of [`crate::events`]
//! (structured loops deliver events in place; the dispatcher-shaped
//! originals accept their loop heads as resume entries).
//!
//! Not replaced: `Rail_FindNearestPoint` (0x335128), see the module report.

use crate::abi::{self, addu, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound, VSlot};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0033_09d8, cell_gather_objects, "Cell_GatherObjects"),
    (0x0033_5960, cell_collide_instance, "Cell_CollideInstance"),
    (0x0033_5b90, cell_collide_object, "Cell_CollideObject"),
    (0x0032_8f28, spatial_key_from_aabb, "SpatialKey_FromAABB"),
    (0x0033_3ef8, overlap_list_collide_volume, "OverlapList_CollideVolume"),
    (0x002c_0778, world_painter_query_update_at_pos, "WorldPainterQuery_UpdateAtPos"),
    (0x002b_83f8, world_sound_query_tree_node, "cWorldSound_QueryTreeNode"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    pub const COLLIDE_VOLUME_OBJECT: Callee = Callee { addr: 0x0033_4888, f: ee::s11::Collide_VolumeObject };
    pub const WORLD_PAINTER_LOOKUP_CELL: Callee =
        Callee { addr: 0x002c_0a10, f: ee::s08::WorldPainterQuery_LookupCell };
    pub const WORLD_SOUND_QUERY_TREE_NODE: Callee =
        Callee { addr: 0x002b_83f8, f: ee::s08::cWorldSound_QueryTreeNode };
    /// Cross-shard direct calls the generator emitted as `call_indirect`.
    pub const AI_FWD_DIFF_CACHE_GET: u32 = 0x0032_79d0;
    pub const AI_FWD_DIFF_CACHE_COLLIDE_ENTRY: u32 = 0x0032_b6e0;
}

/// The identity-matrix rows (`.rodata`) copied into every contact record.
const IDENTITY_ROWS: u32 = 0x004f_f120;

/// Contact record (0x80 bytes) fields written here.
mod contact {
    pub const POS: u32 = 0x00;
    pub const NORMAL: u32 = 0x10;
    pub const ROW0: u32 = 0x20;
    pub const ROW1: u32 = 0x30;
    pub const DEPTH: u32 = 0x40;
    pub const KIND: u32 = 0x44;
    pub const ID: u32 = 0x4c;
    pub const OWNER: u32 = 0x54;
    pub const U: u32 = 0x6c;
    pub const V: u32 = 0x70;
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// `lw` through the CPU load path (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// The address `reg + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lwc1`.
#[inline(always)]
fn fpu(c: &mut Cpu, a: u32) -> FpuF32 {
    Addr(a).read_fpu(c)
}

/// `swc1` of FPU register `f`.
#[inline(always)]
fn store_fpr(c: &mut Cpu, f: usize, a: u32) {
    let v = FpuF32(c.f[f]);
    Addr(a).write_fpu(c, v);
}

/// `sw` of a register.
#[inline(always)]
fn sw_reg(c: &mut Cpu, a: u32, reg: usize) {
    let v = c.g[reg][0];
    c.sw(a, v);
}

/// `p + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A gcc 2.9x virtual call `vt[slot](obj + delta)`: reads the slot (`lh`
/// delta, `lw` function), leaves the function in `func_reg` and the
/// adjusted `this` in a0, and calls through the function table.
#[inline]
fn virtual_call(c: &mut Cpu, obj: u64, vt: u64, slot: u32, func_reg: usize, ret: u32) -> Flow {
    let m = VSlot::read(c, vt, slot);
    c.g[func_reg][0] = m.func;
    c.g[A0][0] = m.this(obj);
    call_ptr(c, m.func as u32, ret)
}

/// One compare of an AABB rejection chain: `f1 = [f1_at]`, `f0 = [f0_at]`,
/// then `c.lt.s`/`c.le.s f0, f1` (or `f1, f0` when `rev`); the chain goes
/// on while the compare holds.
#[derive(Clone, Copy)]
struct Bound {
    f1_at: u32,
    f0_at: u32,
    rev: bool,
}

const fn b(f1_at: u32, f0_at: u32, rev: bool) -> Bound {
    Bound { f1_at, f0_at, rev }
}

/// The six-compare overlap chain the collision code inlines: `first`
/// cycles before the first compare, 5 before each next one, 6 before the
/// last. Leaves f0/f1 and the FCC bit as the original does; true when all
/// six compares hold.
fn overlap(c: &mut Cpu, le: bool, first: u64, tests: [Bound; 6]) -> bool {
    for (i, t) in tests.iter().enumerate() {
        c.cycles += match i {
            0 => first,
            5 => 6,
            _ => 5,
        };
        let f1 = fpu(c, t.f1_at);
        let f0 = fpu(c, t.f0_at);
        c.f[1] = f1.0;
        c.f[0] = f0.0;
        let (x, y) = if t.rev { (f1, f0) } else { (f0, f1) };
        let holds = if le { x.le(y) } else { x.lt(y) };
        abi::set_fcc(c, holds);
        if !holds {
            return false;
        }
    }
    true
}

/// The next free contact record of a collision query (`query+4` records,
/// `query+0xc` count pointer), re-read as the original does: leaves the
/// count pointer in `count_reg`, the array in `base_reg` and the record in
/// `rec_reg`.
fn next_contact(c: &mut Cpu, query: u32, count_reg: usize, base_reg: usize, rec_reg: usize) -> u32 {
    c.g[count_reg][0] = lw(c, p(query, 0xc));
    c.g[base_reg][0] = lw(c, p(query, 4));
    c.g[rec_reg][0] = lw(c, r32(c, count_reg));
    c.g[rec_reg][0] = sx(r32(c, rec_reg) << 7);
    c.g[rec_reg][0] = addu(c.g[rec_reg][0], c.g[base_reg][0]);
    r32(c, rec_reg)
}

// ---------------------------------------------------------------------------
// Cells.

/// `WorldObject` fields.
mod obj {
    pub const NEXT: u32 = 0x00;
    /// 0x20: static (own box at +0x60), 0x40: dynamic (box from the
    /// sub-object's virtual 0x168).
    pub const FLAGS: u32 = 0x08;
    pub const SUB: u32 = 0x0c;
    pub const BOX_MIN: u32 = 0x60;
    pub const BOX_MAX: u32 = 0x6c;
    pub const SHAPE: u32 = 0x88;
}

/// `void Cell_GatherObjects(GatherQuery* q, Cell* cell)` (0x3309d8): walks
/// the cell's object list (`cell+0x20`) and appends every object whose box
/// touches the query box (`q+0xc`, `c.le.s`) to `q->out[(*q->count)++]`.
/// Static objects (flag 0x20) use their own box; dynamic ones (0x40) ask
/// their sub-object (virtual 0x160 "active", 0x168 "box").
pub fn cell_gather_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw(c, at(c, A1, 0x20));
        c.g[S1][0] = c.g[A0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = lw(c, at(c, S0, obj::FLAGS));
            loop {
                c.cycles += 3;
                let flags = c.g[V1][0];
                c.g[V0][0] = flags & 0x40;
                // Some(extra cycles) when the object is taken.
                let take = if flags & 0x20 != 0 {
                    c.g[V0][0] = lw(c, at(c, S1, 0xc));
                    c.g[V1][0] = 0;
                    let (o, q) = (r32(c, S0), r32(c, V0));
                    let hit = overlap(c, true, 7, [
                        b(p(o, obj::BOX_MAX), q, false),
                        b(p(q, 0x10), p(o, obj::BOX_MIN), false),
                        b(p(q, 4), p(o, obj::BOX_MAX + 4), true),
                        b(p(q, 0x14), p(o, obj::BOX_MIN + 4), false),
                        b(p(q, 8), p(o, obj::BOX_MAX + 8), true),
                        b(p(q, 0x18), p(o, obj::BOX_MIN + 8), false),
                    ]);
                    if hit {
                        c.g[V1][0] = 1;
                    }
                    c.cycles += 2;
                    hit.then_some(2)
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        None
                    } else {
                        c.cycles += 8;
                        let sub = lw(c, at(c, S0, obj::SUB));
                        c.g[A1][0] = sub;
                        c.g[V1][0] = lw(c, p(sub as u32, 0xc));
                        let vt = c.g[V1][0];
                        virtual_call(c, sub, vt, 0x160, V0, 0x0033_0ab8)?;
                        if c.g[V0][0] == 0 {
                            None
                        } else {
                            c.cycles += 13;
                            let sub = lw(c, at(c, S0, obj::SUB));
                            c.g[A1][0] = sub;
                            c.g[V1][0] = lw(c, p(sub as u32, 0xc));
                            let vt = c.g[V1][0];
                            virtual_call(c, sub, vt, 0x168, V0, 0x0033_0ad8)?;
                            c.g[V1][0] = lw(c, at(c, S1, 0xc));
                            c.g[A0][0] = 0;
                            let (bx, q) = (r32(c, V0), r32(c, V1));
                            let hit = overlap(c, true, 0, [
                                b(p(bx, 0x10), q, false),
                                b(p(q, 0x10), bx, false),
                                b(p(q, 4), p(bx, 0x14), true),
                                b(p(q, 0x14), p(bx, 4), false),
                                b(p(q, 8), p(bx, 0x18), true),
                                b(p(q, 0x18), p(bx, 8), false),
                            ]);
                            if hit {
                                c.g[A0][0] = 1;
                            }
                            c.cycles += 2;
                            hit.then_some(1)
                        }
                    }
                };
                if let Some(extra) = take {
                    c.cycles += extra;
                    c.g[A1][0] = lw(c, at(c, S1, 4));
                    c.cycles += 8;
                    c.g[A0][0] = lw(c, at(c, S1, 0));
                    c.g[V1][0] = lw(c, r32(c, A1));
                    c.g[V0][0] = sx(r32(c, V1) << 2);
                    c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                    sw_reg(c, r32(c, V0), S0);
                    sw_reg(c, r32(c, A1), V1);
                }
                c.g[S0][0] = lw(c, at(c, S0, obj::NEXT));
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw(c, at(c, S0, obj::FLAGS));
                if event_due(c) {
                    deliver_event_here(c, 0x0033_09f8);
                }
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

/// `void Cell_CollideInstance(CollideQuery* q, Instance* inst)` (0x335960):
/// for a collidable instance (flags `+0xa` & 0x41 == 0x41) whose box
/// (`+0x158`/`+0x164`) strictly overlaps the volume's box (`q->volume`
/// +0x20 max / +0x30 min), looks up the instance's forward-difference
/// entry (`AIFwdDiffCache_Get(q->cache)`), collides the volume with it and
/// on a hit fills the next contact record.
pub fn cell_collide_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        c.g[V1][0] = 0x41;
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = c.lhu(at(c, S1, 0xa)) & 0x41;
        c.g[S0][0] = c.g[A0][0];
        if c.g[V0][0] == c.g[V1][0] {
            collide_instance_body(c, frame)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

fn collide_instance_body(c: &mut Cpu, frame: Frame) -> Flow {
    c.g[V0][0] = lw(c, at(c, S0, 0));
    c.g[V1][0] = 0;
    let (q, i) = (r32(c, V0), r32(c, S1));
    let hit = overlap(c, false, 7, [
        b(p(i, 0x158), p(q, 0x20), true),
        b(p(q, 0x30), p(i, 0x164), true),
        b(p(q, 0x24), p(i, 0x15c), false),
        b(p(q, 0x34), p(i, 0x168), true),
        b(p(q, 0x28), p(i, 0x160), false),
        b(p(q, 0x38), p(i, 0x16c), true),
    ]);
    if hit {
        c.g[V1][0] = 1;
    }
    c.cycles += 2;
    c.g[A1][0] = c.g[S1][0];
    if !hit {
        return Ok(());
    }
    c.cycles += 14;
    // AIFwdDiffCache_Get(q->cache, inst, &entry)
    c.g[A0][0] = lw(c, at(c, S0, 0x14));
    c.g[A2][0] = sx(frame.at(c, 0x30));
    call_ptr(c, callee::AI_FWD_DIFF_CACHE_GET, 0x0033_5a20)?;
    // AIFwdDiffCache_CollideEntry(volume, inst, entry, &pos, &normal, &depth, &uv, q->hint)
    c.g[A0][0] = lw(c, at(c, S0, 0));
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = lw(c, frame.at(c, 0x30));
    c.g[A3][0] = c.g[SP][0];
    c.g[T3][0] = lw(c, at(c, S0, 0x10));
    c.g[T0][0] = sx(frame.at(c, 0x10));
    c.g[T1][0] = sx(frame.at(c, 0x34));
    c.g[T2][0] = sx(frame.at(c, 0x20));
    call_ptr(c, callee::AI_FWD_DIFF_CACHE_COLLIDE_ENTRY, 0x0033_5a44)?;
    let collided = c.g[V0][0] != 0;
    c.g[A0][0] = u64::MAX;
    if !collided {
        return Ok(());
    }
    c.cycles += 75;
    let s0 = r32(c, S0);
    let rec = next_contact(c, s0, V0, A2, V1);
    c.g[A3][0] = 2;
    c.g[A1][0] = sx(IDENTITY_ROWS);
    sw_reg(c, p(rec, contact::KIND), A3);
    sw_reg(c, p(rec, contact::ID), A0);
    c.sw(p(rec, 0x54), 0);
    c.sw(p(rec, 0x50), 0);
    c.sw(p(rec, 0x58), 0);
    sw_reg(c, p(rec, 0x5c), A0);
    sw_reg(c, p(rec, 0x60), A0);
    sw_reg(c, p(rec, 0x64), A0);
    c.lq(V0, IDENTITY_ROWS);
    c.sq(V0, p(rec, contact::ROW0));
    c.lq(A0, IDENTITY_ROWS);
    c.sw(p(rec, 0x70), 0);
    c.sq(A0, p(rec, contact::ROW1));
    c.sw(p(rec, 0x68), 0);
    c.sw(p(rec, 0x6c), 0);

    let rec = next_contact(c, s0, A0, A1, V0);
    c.lq(V1, frame.at(c, 0));
    c.sq(V1, p(rec, contact::POS));
    let rec = next_contact(c, s0, A0, A1, V0);
    c.lq(V1, frame.at(c, 0x10));
    c.sq(V1, p(rec, contact::NORMAL));
    c.g[A1][0] = c.lh(at(c, S1, 0x8));
    let rec = next_contact(c, s0, A0, V1, V0);
    sw_reg(c, p(rec, contact::ID), A1);
    let rec = next_contact(c, s0, V1, A0, V0);
    sw_reg(c, p(rec, contact::OWNER), S1);
    for (from, to) in [(0x34, contact::DEPTH), (0x20, contact::U), (0x24, contact::V)] {
        c.f[0] = fpu(c, frame.at(c, from)).0;
        let rec = next_contact(c, s0, V1, A0, V0);
        store_fpr(c, 0, p(rec, to));
    }
    // ++*q->count
    c.g[V1][0] = lw(c, at(c, S0, 0xc));
    c.g[V0][0] = lw(c, r32(c, V1));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    sw_reg(c, r32(c, V1), V0);
    Ok(())
}

/// `void Cell_CollideObject(CollideQuery* q, WorldObject* o)` (0x335b90):
/// for an object with a shape (`+0x88`) whose box strictly overlaps the
/// volume's (static: own box; dynamic: sub-object virtual 0x160/0x168),
/// `*q->count += Collide_VolumeObject(q->volume, o, &records[count],
/// q->max - count, 2)`.
pub fn cell_collide_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, at(c, S0, obj::SHAPE));
        c.g[S1][0] = c.g[A0][0];
        let extra = if c.g[V0][0] == 0 { 1 } else { collide_object_body(c)? };
        c.cycles += extra;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The tests and the collision; returns the cycles charged on the way out
/// before the epilogue (1 after a collision, else 0).
fn collide_object_body(c: &mut Cpu) -> Result<u64, Unwound> {
    c.cycles += 4;
    c.g[V1][0] = lw(c, at(c, S0, obj::FLAGS));
    let flags = c.g[V1][0];
    c.g[V0][0] = flags & 0x40;
    if flags & 0x20 != 0 {
        c.g[V0][0] = lw(c, at(c, S1, 0));
        c.g[V1][0] = 0;
        let (q, o) = (r32(c, V0), r32(c, S0));
        let hit = overlap(c, false, 7, [
            b(p(o, obj::BOX_MIN), p(q, 0x20), true),
            b(p(q, 0x30), p(o, obj::BOX_MAX), true),
            b(p(q, 0x24), p(o, obj::BOX_MIN + 4), false),
            b(p(q, 0x34), p(o, obj::BOX_MAX + 4), true),
            b(p(q, 0x28), p(o, obj::BOX_MIN + 8), false),
            b(p(q, 0x38), p(o, obj::BOX_MAX + 8), true),
        ]);
        if hit {
            c.g[V1][0] = 1;
        }
        c.cycles += 2;
        if !hit {
            return Ok(0);
        }
        c.cycles += 2;
    } else {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            return Ok(0);
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, at(c, S0, obj::SUB));
        if c.g[A1][0] == 0 {
            return Ok(0);
        }
        c.cycles += 7;
        let sub = c.g[A1][0];
        c.g[V1][0] = lw(c, p(sub as u32, 0xc));
        let vt = c.g[V1][0];
        virtual_call(c, sub, vt, 0x160, V0, 0x0033_5c7c)?;
        if c.g[V0][0] == 0 {
            return Ok(0);
        }
        c.cycles += 13;
        let sub = lw(c, at(c, S0, obj::SUB));
        c.g[A1][0] = sub;
        c.g[V1][0] = lw(c, p(sub as u32, 0xc));
        let vt = c.g[V1][0];
        virtual_call(c, sub, vt, 0x168, V0, 0x0033_5c9c)?;
        c.g[V1][0] = lw(c, at(c, S1, 0));
        c.g[A0][0] = 0;
        let (bx, q) = (r32(c, V0), r32(c, V1));
        let hit = overlap(c, false, 0, [
            b(bx, p(q, 0x20), true),
            b(p(q, 0x30), p(bx, 0x10), true),
            b(p(q, 0x24), p(bx, 4), false),
            b(p(q, 0x34), p(bx, 0x14), true),
            b(p(q, 0x28), p(bx, 8), false),
            b(p(q, 0x38), p(bx, 0x18), true),
        ]);
        if hit {
            c.g[A0][0] = 1;
        }
        c.cycles += 2;
        if !hit {
            return Ok(0);
        }
        c.cycles += 1;
    }
    c.g[V0][0] = lw(c, at(c, S1, 0xc));
    // Collide_VolumeObject(q->volume, o, &records[*count], q->max - *count, 2)
    c.cycles += 14;
    c.g[A1][0] = c.g[S0][0];
    c.g[V1][0] = lw(c, at(c, S1, 8));
    c.g[T0][0] = 2;
    c.g[A3][0] = lw(c, r32(c, V0));
    c.g[T1][0] = lw(c, at(c, S1, 4));
    c.g[A2][0] = sx(r32(c, A3) << 7);
    c.g[A0][0] = lw(c, at(c, S1, 0));
    c.g[A3][0] = sx(r32(c, V1).wrapping_sub(r32(c, A3)));
    c.g[A2][0] = addu(c.g[T1][0], c.g[A2][0]);
    callee::COLLIDE_VOLUME_OBJECT.call(c, 0x0033_5d54)?;
    c.g[A0][0] = lw(c, at(c, S1, 0xc));
    c.g[V1][0] = lw(c, r32(c, A0));
    c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
    sw_reg(c, r32(c, A0), V1);
    Ok(1)
}

// ---------------------------------------------------------------------------
// Overlap lists.

/// `int OverlapList_CollideVolume(OverlapList* l, Volume* v, Contact* out,
/// int max, Hint* hint)` (0x333ef8): collides the volume with every
/// collidable instance of the list (`+0x10c` count, `+0x110` pointers;
/// forward-difference cache of `l->world+0xa4`) and every object with a
/// shape (`+8` count, `+0xc` pointers; `Collide_VolumeObject`), filling
/// `out`; returns the number of contacts.
pub fn overlap_list_collide_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        frame.save(c, S6, 0x70);
        frame.save(c, S7, 0x60);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S5, 0x80);
        frame.save(c, FP, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = 0;
        c.g[S4][0] = c.g[A0][0];
        c.g[S6][0] = 0;
        c.g[S7][0] = c.g[A2][0];
        c.g[V0][0] = lw(c, at(c, S4, 0x10c));
        sw_reg(c, frame.at(c, 0x38), A3);
        sw_reg(c, frame.at(c, 0x3c), T0);
        // Instances.
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = 0x50_0000;
            c.g[S5][0] = u64::MAX;
            c.g[FP][0] = sx(IDENTITY_ROWS);
            c.g[S0][0] = c.g[S7][0];
            c.g[V0][0] = sx(r32(c, S6) << 2);
            loop {
                c.cycles += 7;
                c.g[A0][0] = 0x41;
                c.g[V0][0] = addu(c.g[S4][0], c.g[V0][0]);
                c.g[S1][0] = lw(c, at(c, V0, 0x110));
                c.g[V1][0] = c.lhu(at(c, S1, 0xa)) & 0x41;
                if c.g[V1][0] == c.g[A0][0] {
                    overlap_list_instance(c, frame)?;
                }
                c.g[V0][0] = lw(c, at(c, S4, 0x10c));
                c.cycles += 4;
                c.g[S6][0] = sx(r32(c, S6).wrapping_add(1));
                c.g[V0][0] = (c.g[S6][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, S6) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0033_3f60);
                }
            }
        }
        // Objects.
        c.cycles += 3;
        c.g[V0][0] = lw(c, at(c, S4, 8));
        c.g[S1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, S1) << 2);
            loop {
                c.cycles += 5;
                c.g[V0][0] = addu(c.g[S4][0], c.g[V0][0]);
                c.g[S0][0] = lw(c, at(c, V0, 0xc));
                c.g[V1][0] = lw(c, at(c, S0, obj::SHAPE));
                if c.g[V1][0] != 0 {
                    overlap_list_object(c, frame)?;
                } else {
                    c.g[V0][0] = lw(c, at(c, S4, 8));
                }
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, S1) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0033_40e0);
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// One collidable instance (s1) against the volume (s2); a hit writes the
/// record at s0 and advances s0/s3.
fn overlap_list_instance(c: &mut Cpu, frame: Frame) -> Flow {
    let (v, i) = (r32(c, S2), r32(c, S1));
    c.g[V0][0] = 0;
    let hit = overlap(c, false, 6, [
        b(p(v, 0x20), p(i, 0x158), false),
        b(p(v, 0x30), p(i, 0x164), true),
        b(p(v, 0x24), p(i, 0x15c), false),
        b(p(v, 0x34), p(i, 0x168), true),
        b(p(v, 0x28), p(i, 0x160), false),
        b(p(v, 0x38), p(i, 0x16c), true),
    ]);
    if hit {
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    c.g[A1][0] = c.g[S1][0];
    if hit {
        c.cycles += 15;
        c.g[V0][0] = lw(c, at(c, S4, 0));
        c.g[A2][0] = sx(frame.at(c, 0x30));
        c.g[A0][0] = lw(c, at(c, V0, 0xa4));
        call_ptr(c, callee::AI_FWD_DIFF_CACHE_GET, 0x0033_4014)?;
        c.g[A2][0] = lw(c, frame.at(c, 0x30));
        c.g[A0][0] = c.g[S2][0];
        c.g[T3][0] = lw(c, frame.at(c, 0x3c));
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = sx(frame.at(c, 0x10));
        c.g[T0][0] = sx(frame.at(c, 0x20));
        c.g[T1][0] = sx(frame.at(c, 0x34));
        c.g[T2][0] = c.g[SP][0];
        call_ptr(c, callee::AI_FWD_DIFF_CACHE_COLLIDE_ENTRY, 0x0033_4038)?;
        let collided = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if collided {
            c.cycles += 30;
            let rec = r32(c, S0);
            let sp = r32(c, SP);
            c.sw(p(rec, 0x54), 0);
            sw_reg(c, p(rec, contact::ID), S5);
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
            sw_reg(c, p(rec, contact::KIND), V0);
            c.sw(p(rec, 0x50), 0);
            c.sw(p(rec, 0x58), 0);
            sw_reg(c, p(rec, 0x5c), S5);
            sw_reg(c, p(rec, 0x60), S5);
            sw_reg(c, p(rec, 0x64), S5);
            let rows = r32(c, FP);
            c.lq(V0, rows);
            c.lq(V1, p(sp, 0x10));
            c.lq(A0, p(sp, 0x20));
            c.sq(V0, p(rec, contact::ROW0));
            c.lq(V0, rows);
            c.f[2] = fpu(c, p(sp, 0x34)).0;
            c.sw(p(rec, 0x6c), 0);
            c.sw(p(rec, 0x70), 0);
            c.sq(V1, p(rec, contact::POS));
            c.sq(V0, p(rec, contact::ROW1));
            c.sq(A0, p(rec, contact::NORMAL));
            c.sw(p(rec, 0x68), 0);
            c.f[1] = fpu(c, sp).0;
            c.g[V0][0] = c.lh(at(c, S1, 0x8));
            c.f[0] = fpu(c, p(sp, 4)).0;
            sw_reg(c, p(rec, contact::ID), V0);
            sw_reg(c, p(rec, contact::OWNER), S1);
            store_fpr(c, 2, p(rec, contact::DEPTH));
            store_fpr(c, 1, p(rec, contact::U));
            store_fpr(c, 0, p(rec, contact::V));
            c.g[S0][0] = sx(p(rec, 0x80));
        }
    }
    c.cycles += 1;
    Ok(())
}

/// One object with a shape (s0) against the volume (s2); ends with
/// `v0 = l->object_count` as every path of the original does.
fn overlap_list_object(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = lw(c, at(c, S0, obj::FLAGS));
    let flags = c.g[V1][0];
    c.g[V0][0] = flags & 0x40;
    let collide = if flags & 0x20 != 0 {
        c.g[V0][0] = 0;
        let (v, o) = (r32(c, S2), r32(c, S0));
        let hit = overlap(c, false, 6, [
            b(p(v, 0x20), p(o, obj::BOX_MIN), false),
            b(p(v, 0x30), p(o, obj::BOX_MAX), true),
            b(p(v, 0x24), p(o, obj::BOX_MIN + 4), false),
            b(p(v, 0x34), p(o, obj::BOX_MAX + 4), true),
            b(p(v, 0x28), p(o, obj::BOX_MIN + 8), false),
            b(p(v, 0x38), p(o, obj::BOX_MAX + 8), true),
        ]);
        if hit {
            c.g[V0][0] = 1;
        }
        c.cycles += 2;
        c.g[V0][0] = lw(c, frame.at(c, 0x38));
        if hit {
            c.cycles += 2;
        }
        hit
    } else {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = lw(c, at(c, S4, 8));
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, at(c, S0, obj::SUB));
        if c.g[A1][0] == 0 {
            c.g[V0][0] = lw(c, at(c, S4, 8));
            return Ok(());
        }
        c.cycles += 7;
        let sub = c.g[A1][0];
        c.g[V1][0] = lw(c, p(sub as u32, 0xc));
        let vt = c.g[V1][0];
        virtual_call(c, sub, vt, 0x160, V0, 0x0033_41bc)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = lw(c, at(c, S4, 8));
            return Ok(());
        }
        c.cycles += 12;
        let sub = lw(c, at(c, S0, obj::SUB));
        c.g[A1][0] = sub;
        c.g[V1][0] = lw(c, p(sub as u32, 0xc));
        let vt = c.g[V1][0];
        virtual_call(c, sub, vt, 0x168, V0, 0x0033_41dc)?;
        c.g[V1][0] = 0;
        let (v, bx) = (r32(c, S2), r32(c, V0));
        let hit = overlap(c, false, 0, [
            b(p(v, 0x20), bx, false),
            b(p(v, 0x30), p(bx, 0x10), true),
            b(p(v, 0x24), p(bx, 4), false),
            b(p(v, 0x34), p(bx, 0x14), true),
            b(p(v, 0x28), p(bx, 8), false),
            b(p(bx, 0x18), p(v, 0x38), false),
        ]);
        if hit {
            c.g[V1][0] = 1;
        }
        c.cycles += 2;
        c.g[V0][0] = lw(c, frame.at(c, 0x38));
        if hit {
            c.cycles += 1;
        }
        hit
    };
    if collide {
        // n += Collide_VolumeObject(v, o, &out[n], max - n, 2)
        c.g[A2][0] = sx(r32(c, S3) << 7);
        c.cycles += 7;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = addu(c.g[S7][0], c.g[A2][0]);
        c.g[A3][0] = sx(r32(c, V0).wrapping_sub(r32(c, S3)));
        c.g[A0][0] = c.g[S2][0];
        c.g[T0][0] = 2;
        callee::COLLIDE_VOLUME_OBJECT.call(c, 0x0033_4280)?;
        c.g[S3][0] = addu(c.g[S3][0], c.g[V0][0]);
    }
    c.cycles += 1;
    c.g[V0][0] = lw(c, at(c, S4, 8));
    Ok(())
}

// ---------------------------------------------------------------------------
// Spatial keys.

const SK_ENTRY: u32 = 0x0032_8f28;
/// Loop head (`level++`, halve the cell coordinates): resume entry.
const SK_GROW: u32 = 0x0032_9070;
const SK_END: u32 = 0x0032_91dc;

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
fn spatial_key_initial(c: &mut Cpu) {
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
fn floor_div(c: &mut Cpu, v: FpuF32, cell: FpuF32) {
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
fn spatial_key_grow(c: &mut Cpu) {
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
fn key_bound(c: &mut Cpu, cell: u32, bound_at: u32, upper: bool) -> bool {
    let bound = fpu(c, bound_at);
    c.f[1] = bound.0;
    let corner = FpuF32::from_i32(cell as i32);
    let margin = FpuF32(c.f[3]);
    let size = FpuF32(c.f[2]);
    let edge = (if upper { corner + margin } else { corner - margin }) * size;
    c.f[0] = edge.0;
    let holds = if upper { bound.le(edge) } else { edge.le(bound) };
    abi::set_fcc(c, holds);
    holds
}

/// Does the box fit the key's cell (with the margin)? Leaves t0 = the
/// answer and the scratch registers of the original.
fn spatial_key_fits(c: &mut Cpu) -> bool {
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
// World painter.

const WP_ENTRY: u32 = 0x002c_0778;
/// Loop head (probe the cell list of the current layer): resume entry.
const WP_PROBE: u32 = 0x002c_0810;
const WP_END: u32 = 0x002c_0a10;

#[derive(Clone, Copy)]
enum Wp {
    Probe,
    Release,
    Done,
    Exit,
}

/// `void WorldPainterQuery_UpdateAtPos(WorldPainterQuery* q, float x,
/// float y, float value)` (0x2c0778): moves the query to (x, y) (adding
/// the distance travelled to the painter's accumulator unless it holds the
/// "fresh" constant), then looks up the painted cell of the current layer
/// and feeds the painter (`q->painter`, virtual 0x210 "apply amount",
/// 0x220 "enter cell", 0x228 "release"). Dispatcher-shaped in the
/// original: re-entered at the probe after an unwind.
pub fn world_painter_query_update_at_pos(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x50 };
        let mut stage = match c.pc {
            WP_ENTRY => {
                world_painter_enter(c);
                match world_painter_walk(c) {
                    Some(s) => s,
                    None => return Ok(()),
                }
            }
            WP_PROBE => Wp::Probe,
            pc => {
                c.jump_out(pc, WP_ENTRY, WP_END);
                return Ok(());
            }
        };
        loop {
            stage = match stage {
                Wp::Probe => world_painter_probe(c)?,
                Wp::Release => {
                    c.cycles += 5;
                    let painter = c.g[A1][0];
                    c.g[V0][0] = lw(c, p(painter as u32, 4));
                    let vt = c.g[V0][0];
                    virtual_call(c, painter, vt, 0x228, V1, 0x002c_09f0)?;
                    Wp::Done
                }
                Wp::Done => {
                    c.cycles += 1;
                    c.g[V0][0] = lw(c, at(c, S1, 0));
                    Wp::Exit
                }
                Wp::Exit => break,
            };
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Prologue and the move: s1 = query, s2 = "fresh", f20 = value.
fn world_painter_enter(c: &mut Cpu) {
    c.cycles += 19;
    let frame = Frame::open(c, 0x50);
    c.g[V0][0] = lw(c, gp(c, -0x848));
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save_fpr(c, 20, 0x40);
    c.g[S2][0] = 1;
    frame.save(c, S0, 0x30);
    c.f[20] = c.f[14];
    frame.save_ra(c, 0);
    let fresh_value = fpu(c, gp(c, -0x4284));
    c.f[0] = fresh_value.0;
    c.g[A1][0] = lw(c, at(c, S1, 0));
    c.g[V1][0] = lw(c, at(c, V0, 0x84));
    let acc = fpu(c, r32(c, A1));
    c.f[2] = acc.0;
    c.g[V0][0] = lw(c, at(c, V1, 0x10));
    let fresh = acc.eq_s(fresh_value);
    abi::set_fcc(c, fresh);
    c.g[A0][0] = lw(c, at(c, V0, 8));
    if !fresh {
        c.cycles += 1;
        c.g[S2][0] = 0;
    }
    c.cycles += 2;
    c.g[V1][0] = lw(c, gp(c, 0x770));
    if c.g[S2][0] == 0 {
        // painter->acc += |(x, y) - q->pos|
        c.cycles += 11;
        let dx = fpu(c, at(c, S1, 8)) - FpuF32(c.f[12]);
        let dy = fpu(c, at(c, S1, 0xc)) - FpuF32(c.f[13]);
        let dx2 = dx * dx;
        let dy2 = dy * dy;
        c.f[1] = dy2.0;
        let dist = (dx2 + dy2).sqrt();
        let sum = acc + dist;
        c.f[0] = sum.0;
        Addr(r32(c, A1)).write_fpu(c, sum);
        c.g[V1][0] = lw(c, gp(c, 0x770));
    }
    c.cycles += 5;
    c.g[V0][0] = 0xc;
    store_fpr(c, 12, at(c, S1, 8));
    c.mult(0, A2, V1, V0);
    store_fpr(c, 13, at(c, S1, 0xc));
}

/// The layer record (`a2 = 12 * layer`): probe its cells if it is a
/// populated list (type 3, non-empty), else release. `None` when an event
/// asks the thread to unwind at the probe.
fn world_painter_walk(c: &mut Cpu) -> Option<Wp> {
    c.cycles += 7;
    c.g[V0][0] = lw(c, at(c, A0, 4));
    c.g[A0][0] = 3;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = addu(c.g[A2][0], c.g[V0][0]);
    c.g[V1][0] = c.lh(r32(c, V0));
    c.g[A3][0] = 0;
    if c.g[V1][0] == c.g[A0][0] {
        c.cycles += 3;
        c.g[V0][0] = lw(c, at(c, V0, 8));
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = lw(c, at(c, V0, 4));
            c.g[A3][0] = (0 < c.g[V0][0] as i64) as u64;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = addu(c.g[A1][0], c.g[A2][0]);
    if c.g[A3][0] != 0 {
        if back_edge_unwinds(c, WP_PROBE) {
            return None;
        }
        Some(Wp::Probe)
    } else {
        c.cycles += 1;
        c.g[A1][0] = lw(c, at(c, S1, 0));
        Some(Wp::Release)
    }
}

/// Look up the painted cell at the query position and feed the painter.
fn world_painter_probe(c: &mut Cpu) -> Result<Wp, Unwound> {
    c.cycles += 7;
    c.g[V1][0] = c.lbu(at(c, S1, 0x10));
    c.g[A0][0] = lw(c, at(c, V0, 8));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[A0][0] = addu(c.g[A0][0], c.g[V1][0]);
    c.g[A1][0] = lw(c, at(c, A0, 8));
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        c.g[A1][0] = lw(c, at(c, S1, 0));
        if c.g[S2][0] == 0 {
            return Ok(Wp::Release);
        }
        c.cycles += 7;
        let painter = c.g[A1][0];
        c.g[V0][0] = lw(c, p(painter as u32, 4));
        let vt = c.g[V0][0];
        virtual_call(c, painter, vt, 0x228, V1, 0x002c_0848)?;
        c.g[A1][0] = lw(c, at(c, S1, 0));
        return Ok(Wp::Release);
    }
    c.f[12] = fpu(c, at(c, S1, 8)).0;
    // cell = WorldPainterQuery_LookupCell(q, data)
    c.cycles += 6;
    c.g[A0][0] = c.g[S1][0];
    c.f[13] = fpu(c, at(c, S1, 0xc)).0;
    callee::WORLD_PAINTER_LOOKUP_CELL.call(c, 0x002c_085c)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        c.g[A1][0] = lw(c, at(c, S1, 0));
        return Ok(Wp::Release);
    }
    c.cycles += 5;
    c.g[V0][0] = lw(c, at(c, S1, 0x10));
    c.g[V1][0] = lw(c, at(c, S0, 0));
    c.g[V0][0] = sx(r32(c, V0) >> 8);
    if c.g[V1][0] != c.g[V0][0] {
        c.g[V0][0] = lw(c, at(c, S1, 0));
        return Ok(Wp::Exit);
    }
    // painter->EnterCell(cell->data)
    c.cycles += 9;
    c.g[A2][0] = lw(c, at(c, S1, 0));
    c.g[A1][0] = lw(c, at(c, S0, 4));
    let painter = c.g[A2][0];
    c.g[V1][0] = lw(c, p(painter as u32, 4));
    let vt = c.g[V1][0];
    virtual_call(c, painter, vt, 0x220, V0, 0x002c_0898)?;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = lw(c, at(c, S1, 0));
        c.sw(r32(c, V0), 0);
    }
    c.cycles += 2;
    if c.g[S2][0] != 0 {
        // Fresh: apply -1 and reset the accumulator.
        c.cycles += 12;
        c.g[A2][0] = lw(c, at(c, S1, 0));
        c.g[A1][0] = lw(c, at(c, S0, 4));
        let painter = c.g[A2][0];
        c.g[V0][0] = lw(c, p(painter as u32, 4));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[12] = 0xbf80_0000;
        let vt = c.g[V0][0];
        virtual_call(c, painter, vt, 0x210, V1, 0x002c_08d4)?;
        c.g[V0][0] = lw(c, at(c, S1, 0));
        c.sw(r32(c, V0), 0);
        return Ok(Wp::Done);
    }
    c.cycles += 5;
    let none = fpu(c, gp(c, -0x4280));
    c.f[0] = none.0;
    let unset = FpuF32(c.f[20]).eq_s(none);
    abi::set_fcc(c, unset);
    if !unset {
        // Apply the value.
        c.g[A2][0] = lw(c, at(c, S1, 0));
        c.cycles += 9;
        c.f[12] = c.f[20];
        c.g[A1][0] = lw(c, at(c, S0, 4));
        let painter = c.g[A2][0];
        c.g[V0][0] = lw(c, p(painter as u32, 4));
        let vt = c.g[V0][0];
        virtual_call(c, painter, vt, 0x210, V1, 0x002c_0998)?;
        c.g[V0][0] = lw(c, at(c, S1, 0));
        return Ok(Wp::Exit);
    }
    c.cycles += 7;
    c.g[A1][0] = lw(c, at(c, S0, 4));
    c.f[20] = 0;
    let level = fpu(c, r32(c, A1));
    c.f[1] = level.0;
    let positive = FpuF32(0).le(level);
    abi::set_fcc(c, positive);
    c.g[V0][0] = c.g[A1][0];
    if positive {
        c.cycles += 6;
        c.g[A2][0] = lw(c, at(c, S1, 0));
        let acc = fpu(c, r32(c, A2));
        c.f[0] = acc.0;
        let reached = level.le(acc);
        abi::set_fcc(c, reached);
        if reached {
            // Apply +1 and clear the accumulator.
            c.cycles += 10;
            let painter = c.g[A2][0];
            c.g[V0][0] = lw(c, p(painter as u32, 4));
            c.g[AT][0] = 0x3f80_0000;
            c.f[12] = 0x3f80_0000;
            let vt = c.g[V0][0];
            virtual_call(c, painter, vt, 0x210, V1, 0x002c_0944)?;
            c.g[V0][0] = lw(c, at(c, S1, 0));
            store_fpr(c, 20, r32(c, V0));
            return Ok(Wp::Done);
        }
        c.g[A1][0] = c.g[V0][0];
    } else {
        c.cycles += 2;
        c.g[A2][0] = lw(c, at(c, S1, 0));
        c.g[A1][0] = c.g[V0][0];
    }
    // Apply -level.
    c.cycles += 9;
    let level = fpu(c, r32(c, V0));
    let painter = c.g[A2][0];
    c.g[V0][0] = lw(c, p(painter as u32, 4));
    c.f[12] = (-level).0;
    let vt = c.g[V0][0];
    virtual_call(c, painter, vt, 0x210, V1, 0x002c_0974)?;
    c.g[V0][0] = lw(c, at(c, S1, 0));
    Ok(Wp::Exit)
}

// ---------------------------------------------------------------------------
// World sound octree.

/// The children visited after the first (`node+0`), as (key word changed,
/// change, child offset, return address): the key on the stack walks the
/// eight octants.
const OCTANTS: [(u32, i32, u32, u32); 7] = [
    (4, 1, 0x10, 0x002b_86ec),
    (8, 1, 0x18, 0x002b_8718),
    (4, -1, 0x08, 0x002b_8744),
    (0xc, 1, 0x0c, 0x002b_8770),
    (4, 1, 0x1c, 0x002b_879c),
    (8, -1, 0x14, 0x002b_87c8),
    (4, -1, 0x04, 0x002b_87f4),
];

/// One axis of the cell test: is `pos` above the cell's low edge
/// `(cell - margin) * size` (`c.lt.s pos, edge` fails)?
fn ws_above_low(c: &mut Cpu, cell: u32, pos: FpuF32) -> bool {
    let edge = (FpuF32::from_i32(cell as i32) - FpuF32(c.f[3])) * FpuF32(c.f[1]);
    c.f[0] = edge.0;
    let below = pos.lt(edge);
    abi::set_fcc(c, below);
    !below
}

/// The high edge `(cell + margin) * size` (here `cell` is already +1).
fn ws_below_high(c: &mut Cpu, cell: u32, pos: FpuF32) -> bool {
    let edge = (FpuF32::from_i32(cell as i32) + FpuF32(c.f[3])) * FpuF32(c.f[1]);
    c.f[0] = edge.0;
    let above = edge.lt(pos);
    abi::set_fcc(c, above);
    !above
}

/// `void cWorldSound_QueryTreeNode(a0, List** hits, List** all, Query* q,
/// Node* node, const Key* key)` (0x2b83f8): if the query position lies in
/// the node's cell (`key = {level, x, y, z}`, size `2^level`, margin
/// `gp-0x452c`), appends the node's sound emitters (list `+0x28`, type 5)
/// to `*hits` and, unless the level is 11, recurses into the eight
/// children (`+0..+0x1c`) with their keys built on the stack. (The
/// original's "whole subtree" branch for classification 0 is unreachable:
/// the classification is always 1 or 2.)
pub fn world_sound_query_tree_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x70);
        c.f[3] = fpu(c, gp(c, -0x452c)).0;
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[T0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0x10);
        let key = r32(c, T1);
        c.g[T2][0] = lw(c, p(key, 4));
        c.g[V0][0] = lw(c, key);
        c.g[V1][0] = lw(c, r32(c, S3));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7f) << 23);
        c.f[1] = r32(c, V0);
        c.g[S4][0] = c.g[A0][0];
        let pos = r32(c, V1);
        let px = fpu(c, pos);
        c.f[2] = px.0;
        let inside = 'cell: {
            if !ws_above_low(c, r32(c, T2), px) {
                c.cycles += 1;
                break 'cell false;
            }
            c.cycles += 9;
            c.g[V0][0] = sx(r32(c, T2).wrapping_add(1));
            if !ws_below_high(c, r32(c, V0), px) {
                break 'cell false;
            }
            c.cycles += 10;
            c.g[V0][0] = lw(c, p(key, 8));
            let py = fpu(c, p(pos, 4));
            c.f[2] = py.0;
            if !ws_above_low(c, r32(c, V0), py) {
                break 'cell false;
            }
            c.cycles += 9;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            if !ws_below_high(c, r32(c, V0), py) {
                break 'cell false;
            }
            c.cycles += 10;
            c.g[V0][0] = lw(c, p(key, 0xc));
            let pz = fpu(c, p(pos, 8));
            c.f[2] = pz.0;
            if !ws_above_low(c, r32(c, V0), pz) {
                break 'cell false;
            }
            c.cycles += 9;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            if !ws_below_high(c, r32(c, V0), pz) {
                c.cycles += 1;
                break 'cell false;
            }
            true
        };
        c.g[V1][0] = if inside { 2 } else { 1 };
        c.cycles += 2;
        c.g[V0][0] = 2;
        c.cycles += 2;
        if inside {
            // Append the node's emitters (type 5) to *hits.
            c.cycles += 3;
            c.g[A1][0] = lw(c, at(c, S0, 0x28));
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[A2][0] = 5;
                c.g[V0][0] = lw(c, at(c, A1, 8));
                loop {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A2][0] {
                        c.cycles += 8;
                        c.g[A0][0] = lw(c, at(c, S2, 0));
                        c.g[V1][0] = lw(c, r32(c, A0));
                        c.g[V0][0] = sx(r32(c, V1) << 2);
                        c.g[V0][0] = addu(c.g[A0][0], c.g[V0][0]);
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                        sw_reg(c, at(c, V0, 4), A1);
                        sw_reg(c, r32(c, A0), V1);
                    }
                    c.g[A1][0] = lw(c, at(c, A1, 0));
                    c.cycles += 2;
                    if c.g[A1][0] == 0 {
                        c.cycles += 1;
                        break;
                    }
                    c.g[V0][0] = lw(c, at(c, A1, 8));
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_8638);
                    }
                }
            }
            c.g[A1][0] = lw(c, at(c, T1, 0));
            c.cycles += 3;
            c.g[V0][0] = 0xb;
            let last_level = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
            if !last_level {
                // The first child's key {level - 1, 2x, 2y, 2z} on the stack.
                c.cycles += 12;
                let key = r32(c, T1);
                let sp = sp(c);
                c.g[A0][0] = lw(c, p(key, 0xc));
                c.g[V0][0] = lw(c, p(key, 4));
                c.g[V1][0] = lw(c, p(key, 8));
                c.g[A0][0] = sx(r32(c, A0) << 1);
                c.g[T0][0] = lw(c, at(c, S0, 0));
                c.g[V0][0] = sx(r32(c, V0) << 1);
                c.g[V1][0] = sx(r32(c, V1) << 1);
                sw_reg(c, sp, A1);
                sw_reg(c, p(sp, 4), V0);
                sw_reg(c, p(sp, 8), V1);
                sw_reg(c, p(sp, 0xc), A0);
                if c.g[T0][0] != 0 {
                    c.cycles += 6;
                    world_sound_query_child(c, 0x002b_86c0)?;
                }
                for (word, change, child, ret) in OCTANTS {
                    c.cycles += 5;
                    let at_key = p(sp, word);
                    c.g[V0][0] = lw(c, at_key);
                    c.g[T0][0] = lw(c, at(c, S0, child));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(change as u32));
                    sw_reg(c, at_key, V0);
                    if c.g[T0][0] != 0 {
                        c.cycles += 6;
                        world_sound_query_child(c, ret)?;
                    }
                }
            }
            c.cycles += 1;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Recurse into the child in t0 with the key on the stack.
fn world_sound_query_child(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S3][0];
    c.g[T1][0] = c.g[SP][0];
    callee::WORLD_SOUND_QUERY_TREE_NODE.call(c, ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake_call, machine, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 250;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// A float in [lo, hi).
    fn uf(r: &mut Rng, lo: f32, hi: f32) -> f32 {
        lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
    }

    /// Sub-object vtable: 0x130/0x138 (`Collide_VolumeObject` delegate),
    /// 0x160 (active), 0x168 (box); volume vtable: 0x20 (triangle test);
    /// painter vtable: 0x210, 0x220, 0x228.
    const VT_SUB: u32 = DATA + 0x3_0000;
    const VT_VOL: u32 = DATA + 0x3_0400;
    const VT_PAINTER: u32 = DATA + 0x3_0600;
    /// The box the fake 0x168 returns.
    const BOX2: u32 = DATA + 0x3_0800;

    fn put_vtables(c: &mut Cpu, r: &mut Rng) {
        for (i, off) in [0x130u32, 0x138, 0x160, 0x168].into_iter().enumerate() {
            c.bus.write16(VT_SUB + off, 0);
            c.bus.write32(VT_SUB + off + 4, FAKE + 0x10 * i as u32);
        }
        c.bus.write16(VT_VOL + 0x20, 0);
        c.bus.write32(VT_VOL + 0x24, FAKE + 0x40);
        for (i, off) in [0x210u32, 0x220, 0x228].into_iter().enumerate() {
            c.bus.write16(VT_PAINTER + off, 0);
            c.bus.write32(VT_PAINTER + off + 4, FAKE + 0x50 + 0x10 * i as u32);
        }
        // BOX2: min at +0, max at +0x10.
        for k in 0..3 {
            let lo = uf(r, -5.0, 5.0);
            c.bus.write32(BOX2 + 4 * k, f(lo));
            c.bus.write32(BOX2 + 0x10 + 4 * k, f(lo + uf(r, 0.0, 3.0)));
        }
    }

    /// `fakes::fake_call`, then a result that depends on the method.
    fn fake_varied(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let h = c.g[V1][0];
        match target.wrapping_sub(FAKE) {
            0x00 => {}                                              // delegate (non-null)
            0x10 => c.g[V0][0] = h & 3,                             // contacts added
            0x20 => c.g[V0][0] = ((h >> 8) & 3 != 0) as u64,        // active
            0x30 => c.g[V0][0] = BOX2 as u64,                       // box
            0x40 | 0x60 => c.g[V0][0] = ((h >> 4) & 3 != 0) as u64, // hit / entered
            _ => {}
        }
    }

    fn setup_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.hooks.miss = fake_varied;
        put_vtables(c, &mut r);
        r
    }

    /// A box at `at`: min at `+min`, max at `+max`, random in ±5.
    fn put_box(c: &mut Cpu, r: &mut Rng, at: u32, min: u32, max: u32) {
        for k in 0..3 {
            let lo = uf(r, -5.0, 5.0);
            c.bus.write32(at + min + 4 * k, f(lo));
            c.bus.write32(at + max + 4 * k, f(lo + uf(r, 0.0, 3.0)));
        }
    }

    /// A collision volume at `vol` (box max +0x20, min +0x30 around the
    /// origin, so the zero cells of the cache entry overlap it).
    fn put_volume(c: &mut Cpu, r: &mut Rng, vol: u32) {
        c.bus.write32(vol, 1);
        c.bus.write32(vol + 8, 0);
        for k in 0..3 {
            c.bus.write32(vol + 0x20 + 4 * k, f(uf(r, 1.0, 3.0)));
            c.bus.write32(vol + 0x30 + 4 * k, f(-uf(r, 1.0, 3.0)));
        }
        c.bus.write32(vol + 0x50, VT_VOL);
    }

    const KEY: u32 = 7;

    /// A forward-difference cache at `cache` whose one bucket holds an entry
    /// for `KEY`: cell 0 has a non-sentinel normal, so `CollideEntry`
    /// calls the volume's triangle test once.
    fn put_cache(c: &mut Cpu, cache: u32) {
        let bucket = cache + 0x80;
        let entry = DATA + 0x1_0000;
        c.bus.write32(cache + 4, entry);
        c.bus.write32(cache + 8, bucket);
        c.bus.write32(cache + 0xc, 1);
        c.bus.write32(bucket, entry);
        c.bus.write32(entry, KEY);
        c.bus.write32(entry + 4, entry);
        c.bus.write32(entry + 0x650, f(1.0));
        // CollideEntry's normal sentinel (cells with it are recomputed).
        c.bus.write32(GAME_GP.wrapping_sub(0x2d94), f(-99.0));
    }

    /// A collidable instance at `inst`.
    fn put_instance(c: &mut Cpu, r: &mut Rng, inst: u32, seed: u32) {
        let flags = if seed % 7 == 0 { 0x01 } else { 0x41 | (r.next_u32() & 0xff00) };
        c.bus.write16(inst + 0xa, flags as u16);
        c.bus.write16(inst + 8, r.next_u32() as u16);
        c.bus.write32(inst + 0x150, KEY);
        put_box(c, r, inst, 0x158, 0x164);
    }

    /// A world object at `o` (flags, sub-object with `VT_SUB`, shape).
    fn put_object(c: &mut Cpu, r: &mut Rng, o: u32) {
        let flags = [0x20u32, 0x40, 0x60, 0, 0x21][(r.next_u32() % 5) as usize];
        c.bus.write32(o + 8, flags);
        let sub = o + 0x90;
        c.bus.write32(o + 0xc, if r.next_u32() % 6 == 0 { 0 } else { sub });
        c.bus.write32(sub + 0xc, VT_SUB);
        // Shape record whose type (0) makes Collide_VolumeObject return 0
        // when it has no delegate.
        c.bus.write32(o + 0x88, if r.next_u32() % 6 == 0 { 0 } else { o + 0xc0 });
        put_box(c, r, o, 0x60, 0x6c);
    }

    #[test]
    fn cell_gather_objects_matches() {
        check(ee::s11::f_003309d8__generated, cell_gather_objects, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x3309, 4);
            let (cell, q, qbox, out, count) = (DATA, DATA + 0x1000, DATA + 0x1200, DATA + 0x2000, DATA + 0x1100);
            c.bus.write32(q, out);
            c.bus.write32(q + 4, count);
            c.bus.write32(q + 0xc, qbox);
            c.bus.write32(count, r.next_u32() % 4);
            put_box(c, &mut r, qbox, 0, 0x10);
            let mut next = 0;
            for i in (0..seed % 4).rev() {
                let o = DATA + 0x4000 + 0x100 * i;
                put_object(c, &mut r, o);
                c.bus.write32(o, next);
                next = o;
            }
            c.bus.write32(cell + 0x20, next);
            c.g[A0][0] = sx(q);
            c.g[A1][0] = sx(cell);
        });
    }

    #[test]
    fn cell_collide_instance_matches() {
        check(ee::s11::f_00335960__generated, cell_collide_instance, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x5960, 3);
            let (q, inst, vol, cache, count) = (DATA, DATA + 0x400, DATA + 0x800, DATA + 0xc00, DATA + 0xd00);
            put_volume(c, &mut r, vol);
            put_cache(c, cache);
            put_instance(c, &mut r, inst, seed);
            c.bus.write32(q, vol);
            c.bus.write32(q + 4, DATA + 0x2_0000);
            c.bus.write32(q + 0xc, count);
            c.bus.write32(q + 0x10, 0);
            c.bus.write32(q + 0x14, cache);
            c.bus.write32(count, r.next_u32() % 4);
            c.g[A0][0] = sx(q);
            c.g[A1][0] = sx(inst);
        });
    }

    #[test]
    fn cell_collide_object_matches() {
        check(ee::s11::f_00335b90__generated, cell_collide_object, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x5b90, 4);
            let (q, o, vol, count) = (DATA, DATA + 0x400, DATA + 0x800, DATA + 0xd00);
            put_volume(c, &mut r, vol);
            put_object(c, &mut r, o);
            c.bus.write32(q, vol);
            c.bus.write32(q + 4, DATA + 0x2_0000);
            c.bus.write32(q + 8, 8 + r.next_u32() % 8);
            c.bus.write32(q + 0xc, count);
            c.bus.write32(count, r.next_u32() % 4);
            c.g[A0][0] = sx(q);
            c.g[A1][0] = sx(o);
        });
    }

    #[test]
    fn overlap_list_collide_volume_matches() {
        check(ee::s11::f_00333ef8__generated, overlap_list_collide_volume, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x3ef8, 6);
            let (list, world, vol, cache) = (DATA, DATA + 0x200, DATA + 0x800, DATA + 0xc00);
            put_volume(c, &mut r, vol);
            put_cache(c, cache);
            c.bus.write32(list, world);
            c.bus.write32(world + 0xa4, cache);
            let instances = seed % 3;
            c.bus.write32(list + 0x10c, instances);
            for i in 0..instances {
                let inst = DATA + 0x4000 + 0x200 * i;
                put_instance(c, &mut r, inst, seed + i);
                c.bus.write32(list + 0x110 + 4 * i, inst);
            }
            let objects = (seed / 3) % 3;
            c.bus.write32(list + 8, objects);
            for i in 0..objects {
                let o = DATA + 0x5000 + 0x100 * i;
                put_object(c, &mut r, o);
                c.bus.write32(list + 0xc + 4 * i, o);
            }
            c.g[A0][0] = sx(list);
            c.g[A1][0] = sx(vol);
            c.g[A2][0] = sx(DATA + 0x2_0000);
            c.g[A3][0] = sx(r.next_u32() % 8);
            c.g[T0][0] = 0;
        });
    }

    #[test]
    fn spatial_key_from_aabb_matches() {
        check(ee::s10::f_00328f28__generated, spatial_key_from_aabb, SEEDS, &|c, seed| {
            let mut r = Rng(0x0032_8f28 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = SK_ENTRY;
            let scale = [1.0f32, 0.5, 2.0, 0.01][(seed % 4) as usize];
            c.bus.write32(GAME_GP.wrapping_sub(0x2df8), f(scale));
            c.bus.write32(GAME_GP.wrapping_sub(0x2df4), f(uf(&mut r, 0.0, 0.5)));
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
        });
    }

    #[test]
    fn world_painter_query_update_at_pos_matches() {
        check(ee::s08::f_002c0778__generated, world_painter_query_update_at_pos, SEEDS, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x0778, 4);
            c.pc = WP_ENTRY;
            let (p1, p2, p3, table, recs, list) =
                (DATA + 0x100, DATA + 0x180, DATA + 0x200, DATA + 0x280, DATA + 0x300, DATA + 0x600);
            let (query, painter, data, quad, cells, value) =
                (DATA + 0x800, DATA + 0x900, DATA + 0xa00, DATA + 0xb00, DATA + 0xc00, DATA + 0xd00);
            let (k1, k2) = (uf(&mut r, -1.0, 1.0), uf(&mut r, -1.0, 1.0));
            c.bus.write32(GAME_GP.wrapping_sub(0x848), p1);
            c.bus.write32(p1 + 0x84, p2);
            c.bus.write32(p2 + 0x10, p3);
            c.bus.write32(p3 + 8, table);
            c.bus.write32(table + 4, recs);
            let layer = r.next_u32() % 3;
            c.bus.write32(GAME_GP.wrapping_add(0x770), layer);
            let rec = recs + 12 * layer;
            c.bus.write16(rec, if seed % 9 == 0 { 2 } else { 3 });
            c.bus.write32(rec + 8, if seed % 11 == 0 { 0 } else { list });
            c.bus.write32(list + 4, if seed % 13 == 0 { 0 } else { 1 + r.next_u32() % 3 });
            let k = r.next_u32() % 4;
            let tag = r.next_u32() & 0xff_ffff;
            c.bus.write32(query + 0x10, tag << 8 | k);
            c.bus.write32(list + 8 + 4 * k, if seed % 5 == 0 { 0 } else { data });
            c.bus.write32(data, quad);
            c.bus.write32(data + 8, if seed % 17 == 0 { 0 } else { cells });
            c.bus.write32(cells, if seed % 4 == 1 { tag + 1 } else { tag });
            c.bus.write32(cells + 4, value);
            c.bus.write32(value, f(uf(&mut r, -1.0, 2.0)));
            c.bus.write32(query, painter);
            c.bus.write32(query + 8, f(uf(&mut r, -5.0, 5.0)));
            c.bus.write32(query + 0xc, f(uf(&mut r, -5.0, 5.0)));
            c.bus.write32(painter, f(if seed % 3 == 0 { k1 } else { uf(&mut r, -1.0, 2.0) }));
            c.bus.write32(painter + 4, VT_PAINTER);
            c.bus.write32(GAME_GP.wrapping_sub(0x4284), f(k1));
            c.bus.write32(GAME_GP.wrapping_sub(0x4280), f(k2));
            c.f[12] = f(uf(&mut r, -5.0, 5.0));
            c.f[13] = f(uf(&mut r, -5.0, 5.0));
            c.f[14] = if seed % 2 == 0 { f(k2) } else { f(uf(&mut r, -1.0, 2.0)) };
            c.g[A0][0] = sx(query);
        });
    }

    #[test]
    fn world_sound_query_tree_node_matches() {
        check(ee::s08::f_002b83f8__generated, world_sound_query_tree_node, SEEDS, &|c, seed| {
            let mut r = Rng(0x002b_83f8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x452c), if seed % 2 == 0 { 0 } else { f(0.25) });
            let (key, query, pos, hits, all, root) =
                (DATA, DATA + 0x40, DATA + 0x60, DATA + 0x80, DATA + 0x90, DATA + 0x100);
            let level = [11u32, 2, 3, 1][(seed % 4) as usize];
            c.bus.write32(key, level);
            for k in 1..4 {
                c.bus.write32(key + 4 * k, r.next_u32() % 2);
            }
            c.bus.write32(query, pos);
            for k in 0..3 {
                c.bus.write32(pos + 4 * k, f(uf(&mut r, -2.0, 18.0)));
            }
            c.bus.write32(hits, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000, r.next_u32() % 4);
            // Root: up to eight leaf children, a few emitters.
            for i in 0..8 {
                let child = DATA + 0x200 + 0x40 * i;
                c.bus.write32(root + 4 * i, if r.next_u32() % 3 == 0 { 0 } else { child });
            }
            let mut next = 0;
            for i in 0..(r.next_u32() % 4) {
                let e = DATA + 0x1000 + 0x10 * i;
                c.bus.write32(e, next);
                c.bus.write32(e + 8, if r.next_u32() % 2 == 0 { 5 } else { 4 });
                next = e;
            }
            c.bus.write32(root + 0x28, next);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = sx(hits);
            c.g[A2][0] = sx(all);
            c.g[A3][0] = sx(query);
            c.g[T0][0] = sx(root);
            c.g[T1][0] = sx(key);
        });
    }
}
