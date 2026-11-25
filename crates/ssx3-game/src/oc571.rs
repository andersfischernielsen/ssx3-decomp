//! Staging module oc571 (wave l566c.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc571`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0032_d028,
    sphere_tree_collide_aabb_node,
    "SphereTree_CollideAABBNode",
)];

// SphereTree (`self`): the sphere centres of the eight children at +0x00
// (t4Vector each), the scale at +0x90, the tree info pointer at +0x98.
#[cfg(test)]
const SELF_AXIS: u32 = 0x00;
const SELF_SCALE: u32 = 0x90;
const SELF_TREE_INFO: u32 = 0x98;
// SphereTreeInfo: depth at +0x0c, level table at +0x20, bit tree at +0x28.
const TREE_DEPTH: u32 = 0x0c;
const TREE_LEVELS: u32 = 0x20;
const TREE_BIT_TREE: u32 = 0x28;
// Level entries are 12 bytes: radius (f32) at +0, cenOffset (f32) at +4,
// indexOffset (int) at +8. The loaded cenOffset is `levels[depth + 1]`.
const LEVEL_SIZE: u32 = 0x0c;
const LEVEL_INDEX_OFFSET: u32 = 0x08;
const LEVEL_NEXT_CEN_OFFSET: u32 = LEVEL_SIZE + 0x04;

// Stack slots of the frame (0x110 bytes).
const SLOT_NORMAL: u32 = 0x00;
const SLOT_DIST: u32 = 0x50;
const SLOT_DIR: u32 = 0x10;
const SLOT_CHILD_POS: u32 = 0x20;
const SLOT_SCALED: u32 = 0x30;
const SLOT_TEMP: u32 = 0x40;
const SLOT_OUT_POS: u32 = 0x54;
const SLOT_SAVED_IDX: u32 = 0x58;
const SLOT_RA: u32 = 0x60;
const SLOT_FRAME: u32 = 0x110;

// Calls and return addresses (from the generated body).
const GIS_BOX_SPHERE_INT: u32 = 0x0032_9590;
const RET_AFTER_GIS: u32 = 0x0032_d0b4;
const RET_AFTER_CHILD: u32 = 0x0032_d17c;
const LOOP_HEAD: u32 = 0x0032_d0f8;
const SELF_ADDR: u32 = 0x0032_d028;

/// Address `off(reg)`, read from the register's low word.
#[inline(always)]
fn sphere_reg_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// Epilogue: `c.cycles += 13`, restore the callee-saved registers and `ra`
/// and f20, then `addiu sp, sp, 0x110; jr ra`.
fn sphere_finish(c: &mut Cpu, frame: Frame) {
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
    frame.restore_ra(c, SLOT_RA);
    frame.restore_fpr(c, 20, 0x100);
    frame.close(c);
}

/// The part of the body after the bounding-box test passed: either the
/// recursion into the children (returns 1 at the first child that hits, or
/// 0), or the leaf, which writes the normal and the scaled direction. Every
/// exit then goes to the epilogue (`sphere_finish`).
fn sphere_children_or_leaf(c: &mut Cpu, frame: Frame) -> Flow {
    'leaf_test: {
        c.cycles += 7;
        c.g[A0][0] = Addr(sphere_reg_at(c, S1, SELF_TREE_INFO)).lw(c);
        let tree = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(tree.wrapping_add(TREE_BIT_TREE)).lw(c);
        c.g[V1][0] = Addr(tree.wrapping_add(TREE_DEPTH)).lw(c);
        let bits = c.g[V0][0] as u32;
        c.g[V0][0] = sx(bits.wrapping_add(c.g[S4][0] as u32));
        c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let inside_depth = c.g[V1][0] == 0;
        c.g[S2][0] = Addr(c.g[V0][0] as u32).lbu(c);
        if inside_depth {
            break 'leaf_test;
        }
        c.cycles += 2;
        let no_mask = c.g[S2][0] == 0;
        c.g[V1][0] = 0;
        if no_mask {
            break 'leaf_test;
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(tree.wrapping_add(TREE_LEVELS)).lw(c);
        let saved_stride = c.g[S0][0];
        Addr(sp(c).wrapping_add(SLOT_SAVED_IDX)).sw(c, saved_stride);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let level_next = (c.g[V0][0] as u32).wrapping_add(LEVEL_NEXT_CEN_OFFSET);
        c.lwc1(20, level_next);
        c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);
        // The child loop: i = V1 walks the eight bits of the mask.
        'scan: loop {
            'child: {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let bit_clear = c.g[V0][0] == 0;
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                if bit_clear {
                    break 'child;
                }
                c.cycles += 32;
                // childPos = pos + axis[i] * off, with off = levels[depth+1].cenOffset.
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[A1][0] = c.g[FP][0];
                c.mfc1(3, 20);
                let axis = c.g[V0][0] as u32;
                c.lqc2(4, axis);
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                let temp = sp(c).wrapping_add(SLOT_TEMP);
                c.sqc2(5, temp);
                c.g[A2][0] = c.g[S7][0];
                c.lq(2, temp);
                c.g[A3][0] = sx(sp(c).wrapping_add(SLOT_CHILD_POS));
                c.g[T2][0] = Addr(sp(c).wrapping_add(SLOT_OUT_POS)).lw(c);
                c.g[T0][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                let scaled = sp(c).wrapping_add(SLOT_SCALED);
                c.sq(2, scaled);
                c.g[T3][0] = c.g[S6][0];
                let pos = c.g[S5][0] as u32;
                c.lqc2(3, pos);
                c.lqc2(4, scaled);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, temp);
                c.lq(3, temp);
                c.g[T1][0] = Addr(sphere_reg_at(c, S1, SELF_TREE_INFO)).lw(c);
                c.sq(3, sp(c).wrapping_add(SLOT_CHILD_POS));
                c.g[V1][0] = Addr(sp(c).wrapping_add(SLOT_SAVED_IDX)).lw(c);
                let levels = c.g[T1][0] as u32;
                c.g[V0][0] = Addr(levels.wrapping_add(TREE_LEVELS)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let level = c.g[V0][0] as u32;
                c.g[T1][0] = Addr(level.wrapping_add(LEVEL_INDEX_OFFSET)).lw(c);
                // child base = (i + 1) * indexOffset + base
                c.mult(0, V0, S0, T1);
                c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                let child = Callee {
                    addr: SELF_ADDR,
                    f: ee::s11::f_0032d028__generated,
                };
                child.call(c, RET_AFTER_CHILD)?;
                let child_hit = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if child_hit {
                    return Ok(());
                }
                break 'child;
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[S0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);
            if more {
                if event_due(c) {
                    deliver_event_here(c, LOOP_HEAD);
                }
                continue 'scan;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            return Ok(());
        }
    }
    // Leaf: *outNormal = normal, *outPos = dir * dist.
    c.cycles += 13;
    frame.restore(c, V1, SLOT_NORMAL);
    c.g[V0][0] = 0x1;
    c.lwc1(0, sp(c).wrapping_add(SLOT_DIST));
    let out_normal = c.g[S6][0] as u32;
    c.sq(3, out_normal);
    c.mfc1(4, 0);
    c.lqc2(4, sp(c).wrapping_add(SLOT_DIR));
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let scaled = sp(c).wrapping_add(SLOT_SCALED);
    c.sqc2(5, scaled);
    c.lq(3, scaled);
    c.g[A0][0] = Addr(sp(c).wrapping_add(SLOT_OUT_POS)).lw(c);
    let out_pos = c.g[A0][0] as u32;
    c.sq(3, out_pos);
    c.sq(3, sp(c).wrapping_add(SLOT_CHILD_POS));
    Ok(())
}

/// Body after the prologue: the box test, then the children or the leaf.
fn sphere_collide_body(c: &mut Cpu) -> Flow {
    c.cycles += 37;
    let frame = Frame::open(c, SLOT_FRAME);
    c.g[V0][0] = 0xc;
    frame.save(c, S0, 0xf0);
    frame.save(c, S1, 0xe0);
    frame.save(c, S3, 0xc0);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S4, 0xb0);
    c.g[S3][0] = c.g[T0][0];
    frame.save(c, S5, 0xa0);
    // s0 = depth * 12 (the level stride; v0 = 12).
    c.mult(0, S0, S3, V0);
    frame.save(c, S6, 0x90);
    c.g[S5][0] = c.g[A3][0];
    frame.save(c, S7, 0x80);
    c.g[S4][0] = c.g[T1][0];
    frame.save(c, FP, 0x70);
    c.g[S7][0] = c.g[A2][0];
    frame.save(c, S2, 0xd0);
    c.g[FP][0] = c.g[A1][0];
    frame.save_ra(c, SLOT_RA);
    c.g[S6][0] = c.g[T3][0];
    frame.save_fpr(c, 20, 0x100);
    // gIsBoxSphereInt(boxMin, boxMax, pos, &normal, &dir, &dist, radius * scale)
    c.g[A0][0] = c.g[FP][0];
    c.g[A1][0] = c.g[S7][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[V1][0] = Addr(sphere_reg_at(c, S1, SELF_TREE_INFO)).lw(c);
    c.g[A3][0] = c.g[SP][0];
    c.lwc1(0, sphere_reg_at(c, S1, SELF_SCALE));
    c.g[T0][0] = sx(sp(c).wrapping_add(SLOT_DIR));
    let tree = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(tree.wrapping_add(TREE_LEVELS)).lw(c);
    c.g[T1][0] = sx(sp(c).wrapping_add(SLOT_DIST));
    let out_pos_slot = sp(c).wrapping_add(SLOT_OUT_POS);
    let out_pos_value = c.g[T2][0];
    Addr(out_pos_slot).sw(c, out_pos_value);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let radius_at = c.g[V0][0] as u32;
    c.lwc1(12, radius_at);
    c.mul_s(12, 12, 0);
    call_ptr(c, GIS_BOX_SPHERE_INT, RET_AFTER_GIS)?;
    let missed = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if missed {
        sphere_finish(c, frame);
        return Ok(());
    }
    sphere_children_or_leaf(c, frame)?;
    sphere_finish(c, frame);
    Ok(())
}

/// `int SphereTree_CollideAABBNode(SphereTree* self, t4Vector* boxMin,
/// t4Vector* boxMax, t4Vector* pos, int depth, int base, t4Vector* outPos,
/// t4Vector* outNormal)` (0x32d028): a sphere-box test of the node at
/// `depth`. On a miss it returns 0. On a hit at a leaf (or a node with no
/// children below the tree depth) it stores the normal and the scaled
/// direction and returns 1. Otherwise it recurses into the children whose
/// bit is set in the node's mask and returns 1 at the first child that hits.
pub fn sphere_tree_collide_aabb_node(c: &mut Cpu) {
    guest(c, sphere_collide_body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, put_vec, scramble, Rng, DATA};

    #[test]
    fn sphere_tree_collide_aabb_node_matches() {
        check(
            ssx3_ee::s11::f_0032d028__generated,
            sphere_tree_collide_aabb_node,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_d028 ^ ((seed as u64 + 1) << 24) | 1);
                scramble(c, &mut r);
                let me = DATA;
                let tree = DATA + 0x400;
                let levels = DATA + 0x500;
                let bits = DATA + 0x600;
                let out_pos = DATA + 0x700;
                let out_normal = DATA + 0x720;
                let pos_at = DATA + 0x740;
                let box_min_at = DATA + 0x760;
                let box_max_at = DATA + 0x780;
                // Tree depth 0..2; the incoming depth below or at it (both
                // the leaf and the recursion paths get taken).
                let tree_depth = seed % 3;
                let depth = (seed / 3) % 3;
                let base = (seed / 9) % 16;
                c.bus.write32(me + SELF_TREE_INFO, tree);
                c.bus.write32(tree + TREE_DEPTH, tree_depth);
                c.bus.write32(tree + TREE_LEVELS, levels);
                c.bus.write32(tree + TREE_BIT_TREE, bits);
                c.bus.write32(me + SELF_SCALE, r.f32(2.0));
                for i in 0..8 {
                    put_vec(
                        c,
                        me + SELF_AXIS + i * 0x10,
                        [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0],
                    );
                }
                for k in 0..4 {
                    c.bus.write32(levels + k * LEVEL_SIZE, r.f32(1.0));
                    c.bus.write32(levels + k * LEVEL_SIZE + 4, r.f32(0.5));
                    c.bus.write32(
                        levels + k * LEVEL_SIZE + LEVEL_INDEX_OFFSET,
                        1 + r.next_u32() % 3,
                    );
                }
                fill(c, &mut r, bits, 64);
                let p = [r.f32(4.0), r.f32(4.0), r.f32(4.0), 0];
                put_vec(c, pos_at, p);
                // Most seeds put the box around the centre (a hit); every
                // fourth moves it far away (a miss).
                let far = if seed % 4 == 0 { 100.0 } else { 0.0 };
                let f = |u: u32| f32::from_bits(u);
                let lo = [
                    f(p[0]) - 1.5 + far,
                    f(p[1]) - 1.5 + far,
                    f(p[2]) - 1.5 + far,
                    0.0,
                ];
                let hi = [
                    f(p[0]) + 1.5 + far,
                    f(p[1]) + 1.5 + far,
                    f(p[2]) + 1.5 + far,
                    0.0,
                ];
                put_vec(c, box_min_at, lo.map(f32::to_bits));
                put_vec(c, box_max_at, hi.map(f32::to_bits));
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(box_min_at);
                c.g[A2][0] = sx(box_max_at);
                c.g[A3][0] = sx(pos_at);
                c.g[T0][0] = depth as u64;
                c.g[T1][0] = base as u64;
                c.g[T2][0] = sx(out_pos);
                c.g[T3][0] = sx(out_normal);
            },
        );
    }
}
