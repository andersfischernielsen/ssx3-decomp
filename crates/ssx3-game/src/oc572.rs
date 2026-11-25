//! Staging module oc572 (wave l566d.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc572`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0032_d208,
    sphere_tree_collide_plane_node,
    "SphereTree_CollidePlaneNode",
)];

/// Offsets read by `SphereTree_CollidePlaneNode`: the tree (`self`), its
/// `SphereTreeInfo` and the level records, as the generated body reads them.
pub(crate) mod collide_plane_node {
    /// `float scale`: multiplies a level's radius.
    pub(crate) const SCALE: u32 = 0x90;
    /// `float worstCase`: the closest penetration so far, the distance at
    /// which a leaf last updated the outputs.
    pub(crate) const WORST_CASE: u32 = 0x94;
    /// `SphereTreeInfo* treeInfo`.
    pub(crate) const TREE_INFO: u32 = 0x98;
    /// `int sortAxis[8]`: the order the children are visited in.
    pub(crate) const SORT_AXIS: u32 = 0x9c;
    /// `SphereTreeInfo`: `int depth`.
    pub(crate) const INFO_DEPTH: u32 = 0x0c;
    /// `SphereTreeInfo`: `levels` (12-byte records).
    pub(crate) const INFO_LEVELS: u32 = 0x20;
    /// `SphereTreeInfo`: `unsigned char* bitTree` (one mask per node).
    pub(crate) const INFO_BIT_TREE: u32 = 0x28;
    /// A level record: `float radius` +0, `float cenOffset` +4, `int
    /// indexOffset` +8.
    pub(crate) const LEVEL_SIZE: u32 = 12;
    pub(crate) const LEVEL_RADIUS: u32 = 0;
    pub(crate) const LEVEL_CEN_OFFSET: u32 = 4;
    pub(crate) const LEVEL_INDEX_OFFSET: u32 = 8;
    /// The stack frame size.
    pub(crate) const FRAME: u32 = 0x100;
    /// Where a child call returns to.
    pub(crate) const RET_CHILD: u32 = 0x0032_d368;
    /// The child-loop head (the event check resumes there).
    pub(crate) const LOOP_HEAD: u32 = 0x0032_d2e0;
    /// This function's entry.
    pub(crate) const ENTRY: u32 = 0x0032_d208;
}

/// How the interior walk ended.
#[derive(Clone, Copy, PartialEq, Debug)]
enum CollideWalk {
    /// The node is a leaf (no children, or past the tree's depth): run the
    /// leaf test.
    Leaf,
    /// The children were visited and the loop ended: the epilogue follows
    /// (`s0` already restored).
    Returned,
}

/// How the body ends before the epilogue.
#[derive(Clone, Copy, PartialEq, Debug)]
enum CollideExit {
    /// `s0` already restored: straight to the epilogue.
    Done,
    /// The early and the update exits: one more cycle, then `s0` restored.
    Early,
}

/// `void SphereTree_CollidePlaneNode(SphereTree* self, void* plane,
/// t4Vector* pos, int depth, int base, t4Vector* outA, t4Vector* outB)`
/// (0x0032d208): pushes a plane down the sphere tree. A node whose
/// `plane . pos` lies beyond its radius plus `worstCase` is skipped. An
/// interior node (`depth` above the tree's depth and a non-zero mask) hands
/// the plane to the children its mask selects, nearest-first in `sortAxis`
/// order, each moved by its axis times `cenOffset`. A leaf that lies closer
/// than `worstCase` stores its normal in `outA`, the point on the plane in
/// `outB` and its distance as the new `worstCase`.
pub fn sphere_tree_collide_plane_node(c: &mut Cpu) {
    guest(c, collide_plane_node_body);
}

fn collide_plane_node_body(c: &mut Cpu) -> Flow {
    use collide_plane_node::*;
    c.cycles += 42;
    let frame = Frame::open(c, FRAME);
    c.g[V0][0] = 0xc;
    frame.save(c, 16, 0xe0);
    frame.save(c, 20, 0xa0);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, 21, 0x90);
    c.g[S4][0] = c.g[A1][0];
    frame.save(c, 22, 0x80);
    c.g[S5][0] = c.g[A2][0];
    frame.save(c, 23, 0x70);
    c.g[S6][0] = c.g[A3][0];
    frame.save(c, 30, 0x60);
    // a3 = depth * 12: the byte offset of levels[depth].
    c.mult(0, 7, 22, 2);
    frame.save(c, 17, 0xd0);
    c.g[S7][0] = c.g[T0][0];
    frame.save(c, 18, 0xc0);
    c.g[FP][0] = c.g[T1][0];
    frame.save(c, 19, 0xb0);
    frame.save_ra(c, 0x50);
    frame.save_fpr(c, 20, 0xf0);

    // planeDist = dot(plane, pos) (the unit's VU0 dot sequence).
    c.lqc2(3, c.g[S4][0] as u32);
    c.lqc2(5, c.g[S5][0] as u32);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(3, 4);
    c.mtc1(3, 4);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(TREE_INFO));
    c.mov_s(3, 4);
    c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(SCALE));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(INFO_LEVELS));
    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(WORST_CASE));
    // &levels[depth]; nodeRadius = radius * scale + worstCase.
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(LEVEL_RADIUS));
    c.mul_s(1, 1, 2);
    c.add_s(0, 1, 0);
    // planeDist < nodeRadius is the test that keeps the node.
    c.c_lt_s(3, 0);
    let inside = c.fcc();
    let outb_slot = frame.at(c, 0x40);
    let outb = c.g[T2][0];
    c.sw(outb_slot, outb);

    let exit = if !inside {
        CollideExit::Early
    } else {
        match collide_walk(c, frame)? {
            CollideWalk::Returned => CollideExit::Done,
            CollideWalk::Leaf => collide_leaf(c, frame),
        }
    };
    if exit == CollideExit::Early {
        c.cycles += 1;
        frame.restore(c, 16, 0xe0);
    }

    // Epilogue.
    c.cycles += 12;
    frame.restore(c, 17, 0xd0);
    frame.restore(c, 18, 0xc0);
    frame.restore(c, 19, 0xb0);
    frame.restore(c, 20, 0xa0);
    frame.restore(c, 21, 0x90);
    frame.restore(c, 22, 0x80);
    frame.restore(c, 23, 0x70);
    frame.restore(c, 30, 0x60);
    frame.restore_ra(c, 0x50);
    frame.restore_fpr(c, 20, 0xf0);
    frame.close(c);
    Ok(())
}

/// The interior node: visits the children its mask selects (in `sortAxis`
/// order), or reports a leaf.
fn collide_walk(c: &mut Cpu, frame: Frame) -> Result<CollideWalk, Unwound> {
    use collide_plane_node::*;
    c.cycles += 6;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(INFO_BIT_TREE));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(INFO_DEPTH));
    // The node's mask: bitTree[base].
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
    c.g[V1][0] = ((c.g[S6][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let below_depth = c.g[V1][0] != 0;
    c.g[S3][0] = c.lbu(c.g[V0][0] as u32);
    if !below_depth {
        return Ok(CollideWalk::Leaf);
    }
    c.cycles += 2;
    let has_children = c.g[S3][0] != 0;
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(SORT_AXIS));
    if !has_children {
        return Ok(CollideWalk::Leaf);
    }
    c.cycles += 4;
    // distance = levels[depth + 1].cenOffset (the children's offset).
    c.lwc1(
        20,
        (c.g[A1][0] as u32).wrapping_add(LEVEL_SIZE + LEVEL_CEN_OFFSET),
    );
    let depth12_slot = frame.at(c, 0x44);
    let depth12 = c.g[A3][0];
    c.sw(depth12_slot, depth12);
    c.g[S2][0] = 0x7;
    loop {
        c.cycles += 5;
        c.g[T3][0] = c.lw(c.g[S1][0] as u32); // k = sortAxis[i]
        c.g[V0][0] = sx(((c.g[S3][0] as u32) as i32 >> ((c.g[T3][0] as u32) & 31)) as u32);
        c.g[V0][0] &= 0x1;
        let child_set = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[T3][0] as u32) << 4);
        if child_set {
            c.cycles += 29;
            c.g[A0][0] = c.g[S0][0];
            // &self->axis[k]
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
            c.mfc1(3, 20);
            c.lqc2(4, c.g[V0][0] as u32);
            c.qmtc2(3, 3);
            // childOffset = axis[k] * distance
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            let scaled = frame.at(c, 0x20);
            c.sqc2(5, scaled);
            c.g[A1][0] = c.g[S4][0];
            c.lq(2, frame.at(c, 0x20));
            c.g[A2][0] = c.g[SP][0];
            c.g[T2][0] = c.lw(frame.at(c, 0x40));
            c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
            c.sq(2, frame.at(c, 0x10));
            c.g[T1][0] = c.g[FP][0];
            c.lqc2(3, c.g[S5][0] as u32);
            c.lqc2(4, frame.at(c, 0x10));
            // childPos = pos + childOffset, kept at sp for the child.
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            let child_pos = frame.at(c, 0x20);
            c.sqc2(5, child_pos);
            c.lq(3, frame.at(c, 0x20));
            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(TREE_INFO));
            c.sq(3, sp(c));
            c.g[V1][0] = c.lw(frame.at(c, 0x44));
            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(INFO_LEVELS));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(LEVEL_INDEX_OFFSET));
            // childBase = (k + 1) * indexOffset + base
            c.mult(0, 2, 11, 8);
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
            Callee {
                addr: ENTRY,
                f: ee::s11::SphereTree_CollidePlaneNode,
            }
            .call(c, RET_CHILD)?;
        }
        c.cycles += 3;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
        let more = (c.g[S2][0] as i64) >= 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        if !more {
            c.cycles += 2;
            frame.restore(c, 16, 0xe0);
            return Ok(CollideWalk::Returned);
        }
        // Loop back edge: the event is delivered here; the result is not
        // acted on (the generated code continues the loop either way).
        if event_due(c) {
            deliver_event_here(c, LOOP_HEAD);
        }
    }
}

/// The leaf test: the distance `planeDist - nodeRadius`, and when it is
/// closer than `worstCase`, the outputs and the new `worstCase`.
fn collide_leaf(c: &mut Cpu, frame: Frame) -> CollideExit {
    use collide_plane_node::*;
    c.cycles += 6;
    c.sub_s(20, 4, 1); // distance = planeDist - nodeRadius
    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(WORST_CASE));
    c.c_lt_s(20, 0);
    if !c.fcc() {
        frame.restore(c, 16, 0xe0);
        return CollideExit::Done;
    }
    c.cycles += 29;
    // n = plane with w cleared (the word at sp+0x1c is zeroed).
    c.lq(2, c.g[S4][0] as u32);
    c.sq(2, frame.at(c, 0x10));
    let w_slot = frame.at(c, 0x1c);
    c.sw(w_slot, 0u64);
    c.lq(2, frame.at(c, 0x10));
    c.sq(2, frame.at(c, 0x0));
    // *outA = n * distance
    c.mfc1(3, 20);
    c.lqc2(4, frame.at(c, 0x0));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x20));
    c.lq(2, frame.at(c, 0x20));
    c.sq(2, c.g[FP][0] as u32);
    c.sq(2, frame.at(c, 0x10));
    // *outB = pos - n * planeDist
    c.mfc1(3, 3);
    c.lqc2(4, frame.at(c, 0x0));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x30));
    c.lq(2, frame.at(c, 0x30));
    c.sq(2, frame.at(c, 0x20));
    c.lqc2(3, c.g[S5][0] as u32);
    c.lqc2(4, frame.at(c, 0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x30));
    c.lq(2, frame.at(c, 0x30));
    c.g[V1][0] = c.lw(frame.at(c, 0x40));
    c.sq(2, c.g[V1][0] as u32);
    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(WORST_CASE));
    c.sq(2, frame.at(c, 0x10));
    CollideExit::Early
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::{check, put_vec, scramble, Rng, DATA};

    const SEEDS: u32 = 200;
    const SELF: u32 = DATA + 0x1000;
    const INFO: u32 = DATA + 0x2000;
    const BITS: u32 = DATA + 0x3000;
    const LEVELS: u32 = DATA + 0x4000;
    const PLANE: u32 = DATA + 0x5000;
    const POS: u32 = DATA + 0x5010;
    const OUT_A: u32 = DATA + 0x5100;
    const OUT_B: u32 = DATA + 0x5110;
    const STACK: u32 = DATA + 0x8000;

    /// The tree in memory: a depth 1..3 tree, the axes, the sort order, the
    /// scale and worstCase (seed-selected: early exits, always-update leaves,
    /// random ones), the masks (some seeds all zero) and the plane and point.
    fn fixture_data(c: &mut Cpu, seed: u32, r: &mut Rng) {
        use collide_plane_node::*;
        let depth = 1 + seed % 3;
        c.bus.write32(INFO + INFO_DEPTH, depth);
        c.bus.write32(INFO + INFO_LEVELS, LEVELS);
        c.bus.write32(INFO + INFO_BIT_TREE, BITS);
        for i in 0..=depth {
            let at = LEVELS + i * LEVEL_SIZE;
            let radius = 0.5 + (r.next_u32() % 100) as f32 / 50.0;
            c.bus.write32(at + LEVEL_RADIUS, radius.to_bits());
            c.bus.write32(at + LEVEL_CEN_OFFSET, r.f32(1.0));
            c.bus.write32(at + LEVEL_INDEX_OFFSET, 1 + r.next_u32() % 3);
        }
        for k in 0..8 {
            let v = [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)];
            put_vec(c, SELF + 16 * k, v);
            c.bus.write32(SELF + SORT_AXIS + 4 * k, (k + seed) % 8);
        }
        c.bus
            .write32(SELF + SCALE, (0.5 + (seed % 3) as f32 * 0.25).to_bits());
        let worst = match seed % 7 {
            0 => (-1.0e6f32).to_bits(),  // never inside: the early exit
            1..=4 => 1.0e6f32.to_bits(), // always inside, leaves update
            _ => r.f32(2.0),
        };
        c.bus.write32(SELF + WORST_CASE, worst);
        c.bus.write32(SELF + TREE_INFO, INFO);
        let plane = [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(2.0)];
        put_vec(c, PLANE, plane);
        let pos = [r.f32(2.0), r.f32(2.0), r.f32(2.0), 1.0f32.to_bits()];
        put_vec(c, POS, pos);
        put_vec(c, OUT_A, [0; 4]);
        put_vec(c, OUT_B, [0; 4]);
        for i in 0..0x100 {
            let mask = if seed % 5 == 2 { 0 } else { r.next_u32() as u8 };
            c.bus.write8(BITS + i, mask);
        }
    }

    /// The argument registers. `depth` is below the tree's depth on most
    /// seeds (interior nodes), at it on the rest (leaves).
    fn fixture_args(c: &mut Cpu, seed: u32) {
        let depth = 1 + seed % 3;
        let level = if seed % 4 == 3 { depth } else { seed % depth };
        c.pc = collide_plane_node::ENTRY;
        c.g[A0][0] = sx(SELF);
        c.g[A1][0] = sx(PLANE);
        c.g[A2][0] = sx(POS);
        c.g[A3][0] = level as u64;
        c.g[T0][0] = (seed % 2) as u64;
        c.g[T1][0] = sx(OUT_A);
        c.g[T2][0] = sx(OUT_B);
    }

    #[test]
    fn sphere_tree_collide_plane_node_matches() {
        check(
            ee::s11::f_0032d208__generated,
            sphere_tree_collide_plane_node,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x7a3c_51d9 ^ seed as u64);
                scramble(c, &mut r);
                fixture_data(c, seed, &mut r);
                c.g[SP][0] = sx(STACK);
                fixture_args(c, seed);
            },
        );
    }

    #[test]
    fn sphere_tree_collide_plane_node_events_match() {
        check_events(
            ee::s11::f_0032d208__generated,
            sphere_tree_collide_plane_node,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x3e9b_0d17 ^ seed as u64);
                scramble(c, &mut r);
                fixture_data(c, seed, &mut r);
                let mode = schedule(c, &mut r, seed, collide_plane_node::ENTRY);
                fixture_args(c, seed);
                mode
            },
        );
    }
}
